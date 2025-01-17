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
use syn::spanned::Spanned;
use syn::Ident;

#[derive(Debug, Clone)]
pub struct WalkNodesOutput {
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
			errors: Vec::new(),
			collected_elements: Vec::new(),
			self_closing_elements: self_closing_elements(),
		}
	}
}


type SweetNode = sweet_core::tree::RsxNode<TokenStream>;
type SweetElement = sweet_core::tree::RsxElement<TokenStream>;
type SweetAttribute = sweet_core::tree::RsxAttribute<TokenStream>;

impl WalkNodesOutput {
	#[must_use]
	/// the number of actual html nodes will likely be different
	/// due to fragments, blocks etc
	pub fn visit_nodes(&mut self, nodes: Vec<Node>) -> Vec<SweetNode> {
		nodes
			.into_iter()
			.flat_map(|node| self.visit_node(node))
			.collect()
	}

	/// visit node does not add html to self, giving caller
	/// a decision. Vec is returned to handle fragments
	#[must_use]
	fn visit_node(&mut self, node: Node) -> Vec<SweetNode> {
		match node {
			Node::Doctype(_) => vec![SweetNode::Doctype],
			Node::Text(text) => {
				vec![SweetNode::Text(text.value_string())]
			}
			Node::RawText(raw) => {
				vec![SweetNode::Text(raw.to_string_best())]
			}
			Node::Fragment(NodeFragment { children, .. }) => {
				self.visit_nodes(children)
			}
			Node::Comment(comment) => {
				vec![SweetNode::Comment(comment.value.value())]
			}
			Node::Block(block) => {
				vec![SweetNode::TextBlock(
					quote! {RustParts::TextBlock(#block.to_string())},
				)]
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
				if is_component {
					let props =
						open_tag.attributes.into_iter().map(
							|attr| match attr {
								NodeAttribute::Block(node_block) => {
									quote! {#node_block}
								}
								NodeAttribute::Attribute(attr) => {
									if let Some(value) = attr.value() {
										let key = &attr.key;
										quote! {#key: #value}
									} else {
										let key = attr.key;
										quote! {#key: true}
									}
								}
							},
						);
					let ident = syn::Ident::new(&tag, tag.span());

					let rust = quote! { RustParts::Component(
							#ident{
								#(#props,)*
							}
							.into_rsx_tree()
						)
					};

					let children = self.visit_nodes(children);
					vec![SweetNode::Component(rust, children)]
				} else {
					let attributes = open_tag
						.attributes
						.into_iter()
						.map(|attr| self.visit_attribute(attr))
						.collect();
					let children = self.visit_nodes(children);
					vec![SweetNode::Element(SweetElement {
						tag: tag.clone(),
						attributes,
						children,
						self_closing,
					})]
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

	fn visit_attribute(&mut self, attr: NodeAttribute) -> SweetAttribute {
		match attr {
			NodeAttribute::Block(block) => SweetAttribute::Block(
				quote! {RustParts::AttributeBlock(#block.to_string())},
			),
			NodeAttribute::Attribute(attr) => {
				let key = attr.key.to_string();

				if key.starts_with("on") {
					let tokens = if let Some(value) = attr.value() {
						value.to_token_stream()
					} else {
						// default to a function called onclick
						Ident::new(&key, key.span()).to_token_stream()
					};
					SweetAttribute::BlockValue {
						key,
						value: quote! {RustParts::Event(Box::new(#tokens))},
					}
				} else if let Some(value) = attr.value() {
					match value {
						// only literals (string, number, bool) are not rusty
						syn::Expr::Lit(expr_lit) => {
							let value = match &expr_lit.lit {
								syn::Lit::Str(s) => s.value(),
								other => other.to_token_stream().to_string(),
							};
							SweetAttribute::KeyValue { key, value }
						}
						tokens => SweetAttribute::BlockValue {
							key,
							value: quote! {RustParts::AttributeValue(#tokens.to_string())},
						},
					}
				} else {
					SweetAttribute::Key { key }
				}
			}
		}
	}
}
