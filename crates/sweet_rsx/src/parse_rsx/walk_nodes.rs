use super::self_closing_elements;
use proc_macro2::TokenStream;
use proc_macro2_diagnostics::Diagnostic;
use proc_macro2_diagnostics::Level;
use quote::quote;
use quote::ToTokens;
use rstml::node::Node;
use rstml::node::NodeAttribute;
use rstml::node::NodeElement;
use rstml::node::NodeFragment;
use rstml::node::NodeName;
use std::collections::HashSet;
use sweet_core::rsx::HtmlPartial;
use syn::spanned::Spanned;
use syn::Ident;
use syn::LitStr;

#[derive(Debug, Clone)]
pub struct WalkNodesOutput {
	/// The actual output html
	pub html: HtmlPartial,
	/// The rust identifiers and blocks
	pub rust: Vec<TokenStream>,
	// Additional error and warning messages.
	pub errors: Vec<TokenStream>,
	// Collect elements to provide semantic highlight based on element tag.
	// No differences between open tag and closed tag.
	// Also multiple tags with same name can be present,
	// because we need to mark each of them.
	pub collected_elements: Vec<NodeName>,
	self_closing_elements: HashSet<&'static str>,
}

impl Default for WalkNodesOutput {
	fn default() -> Self {
		Self {
			html: HtmlPartial::default(),
			rust: Vec::new(),
			errors: Vec::new(),
			collected_elements: Vec::new(),
			self_closing_elements: self_closing_elements(),
		}
	}
}


type HtmlNode = sweet_core::rsx::Node;
type HtmlElement = sweet_core::rsx::Element;
type HtmlAttribute = sweet_core::rsx::Attribute;

impl WalkNodesOutput {
	fn extend(&mut self, other: WalkNodesOutput) {
		let WalkNodesOutput {
			html,
			rust,
			errors,
			collected_elements,
			self_closing_elements,
		} = other;
		self.html.extend(html);
		self.rust.extend(rust);
		self.errors.extend(errors);
		self.collected_elements.extend(collected_elements);
		let _ = self_closing_elements; // identical
	}
	fn push_error(&mut self, span: impl Spanned, message: &str) {
		self.errors.push(
			Diagnostic::spanned(span.span(), Level::Error, message)
				.emit_as_expr_tokens(),
		);
	}


	#[must_use]
	/// the number of actual html nodes will likely be different
	/// due to fragments, blocks etc
	pub fn visit_nodes(&mut self, nodes: Vec<Node>) -> Vec<HtmlNode> {
		nodes
			.into_iter()
			.flat_map(|node| self.visit_node(node))
			.collect()
	}

	/// visit node does not add html to self, giving caller
	/// a decision. Vec is returned to handle fragments
	#[must_use]
	fn visit_node(&mut self, node: Node) -> Vec<HtmlNode> {
		match node {
			Node::Doctype(_) => vec![HtmlNode::Doctype],
			Node::Text(text) => {
				vec![HtmlNode::Text(text.value_string())]
			}
			Node::RawText(raw) => {
				vec![HtmlNode::Text(raw.to_string_best())]
			}
			Node::Fragment(NodeFragment { children, .. }) => {
				self.visit_nodes(children)
			}
			Node::Comment(comment) => {
				vec![HtmlNode::Comment(comment.value.value())]
			}
			Node::Block(block) => {
				self.rust
					.push(quote! {RsxRust::InnerText(#block.to_string())});
				vec![HtmlNode::TextBlock]
			}
			Node::Element(el) => {
				self.check_self_closing_children(&el);
				let NodeElement {
					open_tag,
					children,
					close_tag,
				} = el;

				self.collected_elements.push(open_tag.name.clone());
				let self_closing = close_tag.is_none();
				if let Some(close_tag) = close_tag {
					self.collected_elements.push(close_tag.name.clone());
				}
				let tag = open_tag.name.to_string();
				let is_component = tag.starts_with(|c: char| c.is_uppercase());
				let attributes = open_tag
					.attributes
					.into_iter()
					.map(|attr| self.visit_attribute(attr))
					.collect();

				let children = self.visit_nodes(children);

				let el = HtmlElement {
					tag: tag.clone(),
					attributes,
					children,
					self_closing,
				};

				if is_component {
					let ident = syn::Ident::new(&tag, tag.span());
					self.rust.push(
						quote! { RsxRust::Component(#ident.into_parts()) },
					);
					vec![HtmlNode::Component(el)]
				} else {
					vec![HtmlNode::Element(el)]
				}
			}
			Node::Custom(_) => unimplemented!("Custom nodes not yet supported"),
		}
	}

	fn check_self_closing_children<C>(&mut self, element: &NodeElement<C>) {
		if element.children.is_empty()
			|| !self
				.self_closing_elements
				.contains(element.open_tag.name.to_string().as_str())
		{
			return;
		}
		let warning = Diagnostic::spanned(
			element.open_tag.name.span(),
			Level::Warning,
			"Element is processed as empty, and cannot have any child",
		);
		self.errors.push(warning.emit_as_expr_tokens());
	}

	fn visit_attribute(&mut self, attr: NodeAttribute) -> HtmlAttribute {
		match attr {
			NodeAttribute::Block(block) => {
				self.rust.push(block.to_token_stream());
				HtmlAttribute::Block
			}
			NodeAttribute::Attribute(attr) => {
				let key = attr.key.to_string();

				if key.starts_with("on") {
					let tokens = if let Some(value) = attr.value() {
						value.to_token_stream()
					} else {
						// default to a function called onclick
						Ident::new(&key, key.span()).to_token_stream()
					};
					self.rust.push(quote! {RsxRust::Event(Box::new(#tokens))});
					HtmlAttribute::BlockValue { key }
				} else if let Some(value) = attr.value() {
					match value {
						// only literals (string, number, bool) are not rusty
						syn::Expr::Lit(expr_lit) => {
							let value =
								expr_lit.lit.to_token_stream().to_string();
							HtmlAttribute::KeyValue { key, value }
						}
						tokens => {
							self.rust.push(
								quote! {RsxRust::AttributeValue(#tokens.to_string())},
							);
							HtmlAttribute::BlockValue { key }
						}
					}
				} else {
					HtmlAttribute::Key { key }
				}
			}
		}
	}
}
