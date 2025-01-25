use super::self_closing_elements;
use super::RsxAttributeTokens;
use super::RsxNodeTokens;
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
use sweet_core::tokens::RsxRustTokens;
use syn::spanned::Spanned;

#[derive(Debug, Clone)]
pub struct WalkNodes<T> {
	// Additional error and warning messages.
	pub errors: Vec<TokenStream>,
	// Collect elements to provide semantic highlight based on element tag.
	// No differences between open tag and closed tag.
	// Also multiple tags with same name can be present,
	// because we need to mark each of them.
	pub collected_elements: Vec<NodeName>,
	self_closing_elements: HashSet<&'static str>,
	phantom: std::marker::PhantomData<T>,
}

impl<T> Default for WalkNodes<T> {
	fn default() -> Self {
		Self {
			errors: Vec::new(),
			collected_elements: Vec::new(),
			self_closing_elements: self_closing_elements(),
			phantom: std::marker::PhantomData,
		}
	}
}


impl<T:RsxRustTokens> WalkNodes<T> {
	#[must_use]
	/// the number of actual html nodes will likely be different
	/// due to fragments, blocks etc
	pub fn map_nodes<C>(
		&mut self,
		nodes: Vec<Node<C>>,
	) -> Vec<RsxNodeTokens<T>> {
		nodes.into_iter().map(|node| self.map_node(node)).collect()
	}

	/// visit node does not add html to self, giving caller
	/// a decision. Vec is returned to handle fragments
	#[must_use]
	fn map_node<C>(
		&mut self,
		node: Node<C>,
	) -> RsxNodeTokens<T> {
		match node {
			Node::Doctype(_) => RsxNodeTokens::Doctype,
			Node::Text(text) => RsxNodeTokens::Text(text.value_string()),
			Node::RawText(raw) => RsxNodeTokens::Text(raw.to_string_best()),
			Node::Comment(comment) => {
				RsxNodeTokens::Comment(comment.value.value())
			}
			Node::Fragment(NodeFragment { children, .. }) => {
				RsxNodeTokens::Fragment(self.map_nodes(children))
			}
			Node::Block(block) => RsxNodeTokens::Block(block.to_token_stream()),
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
					self.visit_component(&tag, &open_tag.attributes, children)
					// vec![RsxNode::Component(children)]
				} else {
					let attributes = open_tag
						.attributes
						.into_iter()
						.map(|attr| self.visit_attribute(attr))
						.collect();
					let children = self.map_nodes(children);
					RsxNodeTokens::Element {
						tag: tag.clone(),
						attributes,
						children,
						self_closing,
					}
				}
			}
			Node::Custom(_) => unimplemented!("Custom nodes not yet supported"),
		}
	}

	fn visit_component<C>(
		&mut self,
		tag: &str,
		attributes: &Vec<NodeAttribute>,
		children: Vec<Node<C>>,
	) -> RsxNodeTokens<T> {
		let props = attributes.into_iter().map(|attr| match attr {
			NodeAttribute::Block(node_block) => {
				quote! {#node_block}
			}
			NodeAttribute::Attribute(attr) => {
				if let Some(value) = attr.value() {
					let key = &attr.key;
					quote! {#key: #value}
				} else {
					let key = &attr.key;
					quote! {#key: true}
				}
			}
		});
		let ident = syn::Ident::new(&tag, tag.span());
		let children_prop = if children.is_empty() {
			TokenStream::default()
		} else {
			let children = self.map_nodes(children);
			quote! {children: vec![#(#children),*],}
		};

		let rust = quote! {
				#ident{
					#children_prop
					#(#props,)*
				}
				.into_rsx_tree().nodes
		};
		RsxNodeTokens::Component(rust)
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

	fn visit_attribute(&mut self, attr: NodeAttribute) -> RsxAttributeTokens<T> {
		match attr {
			NodeAttribute::Block(block) => {
				RsxAttributeTokens::Block(block.to_token_stream())
			}
			NodeAttribute::Attribute(attr) => match attr.value() {
				None => RsxAttributeTokens::Key {
					key: attr.key.to_string(),
				},
				Some(syn::Expr::Lit(expr_lit)) => {
					let value = match &expr_lit.lit {
						syn::Lit::Str(s) => s.value(),
						other => other.to_token_stream().to_string(),
					};
					RsxAttributeTokens::KeyValue {
						key: attr.key.to_string(),
						value,
					}
				}
				Some(tokens) => RsxAttributeTokens::BlockValue {
					key: attr.key.to_string(),
					value: tokens.to_token_stream(),
				},
			},
		}
	}
}
