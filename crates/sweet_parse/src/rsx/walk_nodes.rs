use super::empty_elements;
use super::RsxPlugin;
use proc_macro2::TokenStream;
use quote::ToTokens;
use rstml::node::Node;
use rstml::node::NodeAttribute;
use rstml::node::NodeName;
use rstml::visitor::visit_attributes;
use rstml::visitor::visit_nodes;
use rstml::visitor::Visitor;
use std::collections::HashSet;
use syn::spanned::Spanned;

pub struct WalkNodes<'a, P> {
	plugin: &'a mut P,
	empty_elements: HashSet<&'static str>,
	output: WalkNodesOutput,
}
impl<'a, P: RsxPlugin> WalkNodes<'a, P> {
	pub fn walk_nodes(
		plugin: &'a mut P,
		mut nodes: Vec<Node>,
	) -> WalkNodesOutput {
		let visitor = Self {
			plugin,
			empty_elements: empty_elements(),
			output: WalkNodesOutput::default(),
		};
		let output = visit_nodes(&mut nodes, visitor);
		output.output
	}

	fn child(&mut self) -> WalkNodes<'_, P> {
		WalkNodes {
			plugin: self.plugin,
			empty_elements: self.empty_elements.clone(),
			output: WalkNodesOutput::default(),
		}
	}

	fn extend_child_output(
		&mut self,
		func: impl FnOnce(WalkNodes<'_, P>) -> WalkNodes<'_, P>,
	) {
		let child = self.child();
		let child = func(child);
		let output = child.output;
		self.output.extend(output);
	}

	fn push_error(&mut self, span: impl Spanned, message: &str) {
		self.output.errors.push(
			proc_macro2_diagnostics::Diagnostic::spanned(
				span.span(),
				proc_macro2_diagnostics::Level::Error,
				message,
			)
			.emit_as_expr_tokens(),
		);
	}
}

#[derive(Default)]
pub struct WalkNodesOutput {
	/// The actual output html
	pub html_string: String,
	/// The event handlers
	pub rust_events: Vec<TokenStream>,
	/// Rust blocks blocks
	pub rust_blocks: Vec<TokenStream>,
	// Additional error and warning messages.
	pub errors: Vec<TokenStream>,
	// Collect elements to provide semantic highlight based on element tag.
	// No differences between open tag and closed tag.
	// Also multiple tags with same name can be present,
	// because we need to mark each of them.
	pub collected_elements: Vec<NodeName>,
}

impl WalkNodesOutput {
	fn extend(&mut self, other: WalkNodesOutput) {
		let WalkNodesOutput {
			html_string,
			rust_events,
			rust_blocks,
			errors,
			collected_elements,
		} = other;
		self.html_string.push_str(&html_string);
		self.rust_blocks.extend(rust_blocks);
		self.errors.extend(errors);
		self.collected_elements.extend(collected_elements);
		self.rust_events.extend(rust_events);
	}
}
impl<'a, P> syn::visit_mut::VisitMut for WalkNodes<'a, P> {}

impl<'a, P: RsxPlugin, C> Visitor<C> for WalkNodes<'a, P>
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
		self.extend_child_output(|child| {
			visit_nodes(&mut fragment.children, child)
		});
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
		// we dont visit blocks, they are handled by visit_element
		// because it needs relative context
		// self.output.html_string.push_str("{}");
		// self.output.rust_blocks.push(block.to_token_stream());
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

		if let Err(err) = self.plugin.visit_element(element, &mut self.output) {
			self.output.errors.push(err.to_compile_error());
		}



		self.extend_child_output(|child| {
			visit_attributes(element.attributes_mut(), child)
		});

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
				self.output.errors.push(warning.emit_as_expr_tokens())
			}

			return false;
		}
		self.extend_child_output(|child| {
			visit_nodes(&mut element.children, child)
		});

		self.output.html_string.push_str(&format!("</{}>", name));
		false
	}
	fn visit_attribute(&mut self, attr: &mut NodeAttribute) -> bool {
		log_visit("ATTRIBUTE", &attr);
		// attributes
		match attr {
			NodeAttribute::Block(block) => {
				self.push_error(
					block.span(),
					"block attributes not yet supported",
				);
				// log_visit("ATTRIBUTE - BLOCK", block.clone());
				// If the nodes parent is an attribute we prefix with whitespace
				// self.output.html_string.push(' ');
				// self.output.html_string.push_str("{}");
				// self.output.rust_blocks.push(block.to_token_stream());
			}
			NodeAttribute::Attribute(attr) => {
				let key_str = attr.key.to_string();
				if key_str.starts_with("on") {
					if let Err(err) =
						self.plugin.visit_event(attr, &mut self.output)
					{
						self.output.errors.push(err.to_compile_error());
					}
				} else {
					// log_visit("ATTRIBUTE - VANILLA", attribute.clone());
					self.output.html_string.push_str(&format!(" {}", attr.key));
					if let Some(value) = attr.value() {
						self.push_error(
							value.span(),
							"attribute values not yet supported",
						);
					}
				}
			}
		}
		false
	}
}


#[allow(unused)]
fn log_visit(prefix: &str, tokens: impl ToTokens) {
	// crashes rust analyzer?
	// println!("🚀 {}: {}", prefix, tokens.into_token_stream().to_string());
}
