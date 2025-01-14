use super::empty_elements;
use proc_macro2::TokenStream;
use quote::ToTokens;
use rstml::node::KeyedAttribute;
use rstml::node::Node;
use rstml::node::NodeAttribute;
use rstml::node::NodeName;
use rstml::visitor::visit_attributes;
use rstml::visitor::visit_nodes;
use rstml::visitor::Visitor;
use std::collections::HashSet;
use syn::spanned::Spanned;
use syn::Ident;


pub struct WalkNodes {
	empty_elements: HashSet<&'static str>,
	output: WalkNodesOutput,
}
impl WalkNodes {
	pub fn walk_nodes(mut nodes: Vec<Node>) -> WalkNodesOutput {
		let visitor = WalkNodes {
			empty_elements: empty_elements(),
			output: WalkNodesOutput::default(),
		};
		let output = visit_nodes(&mut nodes, visitor);
		output.output
	}
	fn child_output(&self) -> Self {
		Self {
			empty_elements: self.empty_elements.clone(),
			output: WalkNodesOutput::default(),
		}
	}
}


// ⚠️ WHENEVER ADDING VALUES UPDATE EXTEND
#[derive(Default)]
pub struct WalkNodesOutput {
	/// The actual output html
	pub html_string: String,
	/// The event handlers
	pub rust_events: Vec<TokenStream>,
	/// The blocks
	pub rust_blocks: Vec<TokenStream>,
	// Additional diagnostic messages.
	pub diagnostics: Vec<TokenStream>,
	// Collect elements to provide semantic highlight based on element tag.
	// No differences between open tag and closed tag.
	// Also multiple tags with same name can be present,
	// because we need to mark each of them.
	pub collected_elements: Vec<NodeName>,
}


#[allow(unused)]
fn log_visit(prefix: &str, tokens: impl ToTokens) {
	// crashes rust analyzer?
	// println!("🚀 {}: {}", prefix, tokens.into_token_stream().to_string());
}

impl WalkNodesOutput {
	fn extend(&mut self, other: WalkNodesOutput) {
		self.html_string.push_str(&other.html_string);
		self.rust_blocks.extend(other.rust_blocks);
		self.diagnostics.extend(other.diagnostics);
		self.collected_elements.extend(other.collected_elements);
		self.rust_events.extend(other.rust_events);
	}
}
impl syn::visit_mut::VisitMut for WalkNodes {}

impl<C> Visitor<C> for WalkNodes
where
	C: rstml::node::CustomNode + 'static,
{
	fn visit_doctype(
		&mut self,
		doctype: &mut rstml::node::NodeDoctype,
	) -> bool {
		log_visit("DOCTYPE", &doctype);
		let value = &doctype.value.to_token_stream_string();
		self.output
			.html_string
			.push_str(&format!("<!DOCTYPE {}>", value));
		false
	}
	fn visit_text_node(&mut self, node: &mut rstml::node::NodeText) -> bool {
		log_visit("TEXT", &node);
		self.output.html_string.push_str(&node.value_string());
		false
	}
	fn visit_raw_node<OtherC: rstml::node::CustomNode>(
		&mut self,
		node: &mut rstml::node::RawText<OtherC>,
	) -> bool {
		log_visit("RAW", &node);
		self.output.html_string.push_str(&node.to_string_best());
		false
	}
	fn visit_fragment(
		&mut self,
		fragment: &mut rstml::node::NodeFragment<C>,
	) -> bool {
		log_visit("FRAGMENT", &fragment);
		let visitor = self.child_output();
		let child_output = visit_nodes(&mut fragment.children, visitor);
		self.output.extend(child_output.output);
		false
	}

	fn visit_comment(
		&mut self,
		comment: &mut rstml::node::NodeComment,
	) -> bool {
		log_visit("COMMENT", &comment);
		self.output
			.html_string
			.push_str(&format!("<!-- {} -->", comment.value.value()));
		false
	}
	fn visit_block(&mut self, block: &mut rstml::node::NodeBlock) -> bool {
		log_visit("BLOCK", &block);
		self.output.html_string.push_str("{}");
		self.output.rust_blocks.push(block.to_token_stream());
		false
	}
	fn visit_element(
		&mut self,
		element: &mut rstml::node::NodeElement<C>,
	) -> bool {
		log_visit("ELEMENT", &element);
		let name = element.name().to_string();
		self.output.html_string.push_str(&format!("<{}", name));
		self.output
			.collected_elements
			.push(element.open_tag.name.clone());
		if let Some(e) = &element.close_tag {
			self.output.collected_elements.push(e.name.clone())
		}

		let visitor = self.child_output();
		let attribute_visitor =
			visit_attributes(element.attributes_mut(), visitor);
		self.output.extend(attribute_visitor.output);

		self.output.html_string.push('>');

		// Ignore childs of special Empty elements
		if self
			.empty_elements
			.contains(element.open_tag.name.to_string().as_str())
		{
			self.output
				.html_string
				.push_str(&format!("/</{}>", element.open_tag.name));
			if !element.children.is_empty() {
				let warning = proc_macro2_diagnostics::Diagnostic::spanned(
					element.open_tag.name.span(),
					proc_macro2_diagnostics::Level::Warning,
					"Element is processed as empty, and cannot have any child",
				);
				self.output.diagnostics.push(warning.emit_as_expr_tokens())
			}

			return false;
		}
		// children

		let visitor = self.child_output();
		let child_output = visit_nodes(&mut element.children, visitor);
		self.output.extend(child_output.output);
		self.output.html_string.push_str(&format!("</{}>", name));
		false
	}
	fn visit_attribute(&mut self, attribute: &mut NodeAttribute) -> bool {
		log_visit("ATTRIBUTE", &attribute);
		// attributes
		match attribute {
			NodeAttribute::Block(block) => {
				// log_visit("ATTRIBUTE - BLOCK", block.clone());
				// If the nodes parent is an attribute we prefix with whitespace
				self.output.html_string.push(' ');
				self.output.html_string.push_str("{}");
				self.output.rust_blocks.push(block.to_token_stream());
			}
			NodeAttribute::Attribute(attribute) => {
				let key_str = attribute.key.to_string();
				if key_str.starts_with("on") {
					self.visit_event(attribute);
				} else {
					// log_visit("ATTRIBUTE - VANILLA", attribute.clone());
					self.output
						.html_string
						.push_str(&format!(" {}", attribute.key));
					if let Some(value) = attribute.value() {
						self.output.html_string.push_str(r#"="{}""#);
						self.output.rust_blocks.push(value.to_token_stream());
					}
				}
			}
		}
		false
	}
}

impl WalkNodes {
	fn visit_event(&mut self, attr: &KeyedAttribute) {
		let Some(value) = attr.value() else {
			self.output.diagnostics.push(
				proc_macro2_diagnostics::Diagnostic::spanned(
					attr.span(),
					proc_macro2_diagnostics::Level::Error,
					"Event handler must have a value",
				)
				.emit_as_expr_tokens(),
			);
			return;
		};
		// match value {
		// 	Expr::Closure(expr_closure) => todo!(),
		// 	_ => todo!(),
		// }
		// self.output.dynamic_attributes.push(rehydrate);
		// self.output.values.push(rehydrate.to_token_stream());
		let index = self.output.rust_events.len();
		self.output
			.html_string
			.push_str(&format!("{}=\"a-{}\"", attr.key, index));

		let ident = Ident::new(&format!("a_{}", index), attr.span());

		let rehydrate = quote::quote! {
			let #ident = #value;
		};
		self.output.rust_events.push(rehydrate);
	}
}
