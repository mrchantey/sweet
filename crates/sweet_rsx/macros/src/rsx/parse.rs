use super::WalkNodesOutput;
use proc_macro::TokenStream;
use proc_macro2_diagnostics::Diagnostic;
use quote::quote;
use quote::quote_spanned;
use rstml::node::Node;
use rstml::node::NodeName;
use rstml::Parser;
use rstml::ParserConfig;
use std::collections::HashSet;
use syn::spanned::Spanned;

pub fn empty_elements() -> HashSet<&'static str> {
	[
		"area", "base", "br", "col", "embed", "hr", "img", "input", "link",
		"meta", "param", "source", "track", "wbr",
	]
	.into_iter()
	.collect()
}


pub fn parse_nodes(tokens: TokenStream) -> (Vec<Node>, Vec<Diagnostic>) {
	let empty_elements = empty_elements();
	let config = ParserConfig::new()
		.recover_block(true)
		.always_self_closed_elements(empty_elements)
		.raw_text_elements(["script", "style"].into_iter().collect())
		.macro_call_pattern(quote!(html! {%%}));

	let parser = Parser::new(config);
	let (nodes, errors) = parser.parse_recoverable(tokens).split_vec();

	(nodes, errors)
}



pub fn collect_nodes(
	WalkNodesOutput {
		// collected_elements,
		diagnostics,
		html_string,
		dynamic_attributes,
		values,
		..
	}: WalkNodesOutput,
	errors: Vec<Diagnostic>,
) -> TokenStream {
	// let docs = generate_tags_docs(&collected_elements);
	let errors = errors
		.into_iter()
		.map(|e| e.emit_as_expr_tokens())
		.chain(diagnostics);
	quote! {
		// {
		// 	#(#dynamic_attributes),*
		// }
		{
			// Make sure that "compile_error!(..);"  can be used in this context.
			#(#errors;)*
			// Make sure that "enum x{};" and "let _x = crate::element;"  can be used in this context
			// #(#docs;)*
			format!(#html_string, #(#values),*)
		}
	}
	.into()
}

pub fn _generate_tags_docs(
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
