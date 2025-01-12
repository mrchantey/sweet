use proc_macro::TokenStream;
use quote::quote;
use quote::quote_spanned;
use quote::ToTokens;
use rstml::node::Node;
use rstml::node::NodeAttribute;
use rstml::node::NodeName;
use rstml::visitor::visit_attributes;
use rstml::visitor::visit_nodes;
use rstml::visitor::Visitor;
use rstml::Parser;
use rstml::ParserConfig;
use std::collections::HashSet;
use syn::spanned::Spanned;
// mod escape;
#[derive(Default)]
struct WalkNodesOutput {
	static_format: String,
	// Use proc_macro2::TokenStream instead of syn::Expr
	// to provide more errors to the end user.
	values: Vec<proc_macro2::TokenStream>,
	// Additional diagnostic messages.
	diagnostics: Vec<proc_macro2::TokenStream>,
	// Collect elements to provide semantic highlight based on element tag.
	// No differences between open tag and closed tag.
	// Also multiple tags with same name can be present,
	// because we need to mark each of them.
	collected_elements: Vec<NodeName>,
}
struct WalkNodes<'a> {
	empty_elements: &'a HashSet<&'a str>,
	output: WalkNodesOutput,
}
impl<'a> WalkNodes<'a> {
	fn child_output(&self) -> Self {
		Self {
			empty_elements: self.empty_elements,
			output: WalkNodesOutput::default(),
		}
	}
}

fn log_visit(prefix: &str, tokens: impl ToTokens) {
	println!("🚀 {}: {}", prefix, tokens.into_token_stream().to_string());
}

impl WalkNodesOutput {
	fn extend(&mut self, other: WalkNodesOutput) {
		self.static_format.push_str(&other.static_format);
		self.values.extend(other.values);
		self.diagnostics.extend(other.diagnostics);
		self.collected_elements.extend(other.collected_elements);
	}
}
impl<'a> syn::visit_mut::VisitMut for WalkNodes<'a> {}

impl<'a, C> Visitor<C> for WalkNodes<'a>
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
			.static_format
			.push_str(&format!("<!DOCTYPE {}>", value));
		false
	}
	fn visit_text_node(&mut self, node: &mut rstml::node::NodeText) -> bool {
		log_visit("TEXT", &node);
		self.output.static_format.push_str(&node.value_string());
		false
	}
	fn visit_raw_node<OtherC: rstml::node::CustomNode>(
		&mut self,
		node: &mut rstml::node::RawText<OtherC>,
	) -> bool {
		log_visit("RAW", &node);
		self.output.static_format.push_str(&node.to_string_best());
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
			.static_format
			.push_str(&format!("<!-- {} -->", comment.value.value()));
		false
	}
	fn visit_block(&mut self, block: &mut rstml::node::NodeBlock) -> bool {
		log_visit("BLOCK", &block);
		self.output.static_format.push_str("{}");
		self.output.values.push(block.to_token_stream());
		false
	}
	fn visit_element(
		&mut self,
		element: &mut rstml::node::NodeElement<C>,
	) -> bool {
		log_visit("ELEMENT", &element);
		let name = element.name().to_string();
		self.output.static_format.push_str(&format!("<{}", name));
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

		self.output.static_format.push('>');

		// Ignore childs of special Empty elements
		if self
			.empty_elements
			.contains(element.open_tag.name.to_string().as_str())
		{
			self.output
				.static_format
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
		self.output.static_format.push_str(&format!("</{}>", name));
		false
	}
	fn visit_attribute(&mut self, attribute: &mut NodeAttribute) -> bool {
		log_visit("ATTRIBUTE", &attribute);
		// attributes
		match attribute {
			NodeAttribute::Block(block) => {
				// log_visit("ATTRIBUTE - BLOCK", block.clone());
				// If the nodes parent is an attribute we prefix with whitespace
				self.output.static_format.push(' ');
				self.output.static_format.push_str("{}");
				self.output.values.push(block.to_token_stream());
			}
			NodeAttribute::Attribute(attribute) => {
				// log_visit("ATTRIBUTE - VANILLA", attribute.clone());
				self.output
					.static_format
					.push_str(&format!(" {}", attribute.key));
				if let Some(_value) = attribute.value() {
					self.output.static_format.push_str(r#"="event-handler-1""#);
					// self.output.static_format.push_str(r#"="{}""#);
					// self.output.values.push(value.to_token_stream());
				}
			}
		}
		false
	}
}
fn walk_nodes<'a>(
	empty_elements: &'a HashSet<&'a str>,
	nodes: &'a mut [Node],
) -> WalkNodesOutput {
	let visitor = WalkNodes {
		empty_elements,
		output: WalkNodesOutput::default(),
	};
	let mut nodes = nodes.to_vec();
	let output = visit_nodes(&mut nodes, visitor);
	output.output
}

pub fn html_inner(tokens: TokenStream, ide_helper: bool) -> TokenStream {
	// https://developer.mozilla.org/en-US/docs/Glossary/Empty_element
	let empty_elements: HashSet<_> = [
		"area", "base", "br", "col", "embed", "hr", "img", "input", "link",
		"meta", "param", "source", "track", "wbr",
	]
	.into_iter()
	.collect();
	let config = ParserConfig::new()
		.recover_block(true)
		.always_self_closed_elements(empty_elements.clone())
		.raw_text_elements(["script", "style"].into_iter().collect())
		.macro_call_pattern(quote!(html! {%%}));

	let parser = Parser::new(config);
	let (mut nodes, errors) = parser.parse_recoverable(tokens).split_vec();

	let WalkNodesOutput {
		static_format: html_string,
		values,
		collected_elements: elements,
		diagnostics,
	} = walk_nodes(&empty_elements, &mut nodes);
	let docs = if ide_helper {
		generate_tags_docs(&elements)
	} else {
		vec![]
	};
	let errors = errors
		.into_iter()
		.map(|e| e.emit_as_expr_tokens())
		.chain(diagnostics);
	quote! {
		{
			// Make sure that "compile_error!(..);"  can be used in this context.
			#(#errors;)*
			// Make sure that "enum x{};" and "let _x = crate::element;"  can be used in this context
			#(#docs;)*
			format!(#html_string, #(#values),*)
		}
	}
	.into()
}

pub fn generate_tags_docs(
	elements: &[NodeName],
) -> Vec<proc_macro2::TokenStream> {
	// Mark some of elements as type,
	// and other as elements as fn in crate::docs,
	// to give an example how to link tag with docs.
	let elements_as_type: HashSet<&'static str> =
		vec!["html", "head", "meta", "link", "body"]
			.into_iter()
			.collect();

	elements
		.into_iter()
		.map(|e| {
			if elements_as_type.contains(&*e.to_string()) {
				let element = quote_spanned!(e.span() => enum);
				quote!({#element X{}})
			} else {
				// let _ = crate::docs::element;
				let element = quote_spanned!(e.span() => element);
				quote!(let _ = crate::docs::#element)
			}
		})
		.collect()
}
