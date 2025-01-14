use super::parse_rstml;
use super::RsxPlugin;
use super::WalkNodes;
use proc_macro2::TokenStream;
use proc_macro2_diagnostics::Diagnostic;
use syn::spanned::Spanned;
use syn::Expr;

#[derive(Debug)]
pub struct RsxPartsTokens {
	/// the raw html, in the case of inline will also be in the rust block
	pub html: String,
	/// the raw css, in the case of inline will also be in the rust block
	pub css: String,
	/// the hydration instructions to replace the macro with.
	pub expr: Expr,
	// TODO css
	/// errors that occurred while parsing the rsx macro.
	pub rstml_errors: Vec<Diagnostic>,
	/// errors that occurred while walking the html nodes.
	pub node_walk_errors: Vec<TokenStream>,
	/// errors that occured during [RsxPlugin::parse]
	pub rsx_plugin_errors: Vec<syn::Error>,
}

#[cfg(test)]
impl Default for RsxPartsTokens {
	fn default() -> Self {
		Self {
			html: String::new(),
			css: String::new(),
			expr: syn::parse_quote!({}),
			rstml_errors: Vec::new(),
			node_walk_errors: Vec::new(),
			rsx_plugin_errors: Vec::new(),
		}
	}
}

impl RsxPartsTokens {
	/// Passed in the contents of an rsx! macro.
	pub fn parse(plugin: &mut impl RsxPlugin, tokens: TokenStream) -> Self {
		let span = tokens.span();
		let (nodes, errors) = parse_rstml(tokens);

		let output = WalkNodes::walk_nodes(plugin, nodes);

		let blocks = output.rust_blocks;
		let events = output.rust_events;

		// TODO this should be parsed into the hydrate step
		let expr = syn::parse_quote_spanned! {span=> sweet::prelude::Hydrated {
			events:	vec![#(#events),*],
			blocks: vec![#(#blocks),*]
		}};

		Self {
			html: output.html_string,
			css: String::new(), //todo
			expr,
			rstml_errors: errors,
			node_walk_errors: output.errors,
			rsx_plugin_errors: Vec::new(),
		}
	}
}




// #[cfg(test)]
// mod test {
// 	use crate::prelude::*;
// 	use sweet::prelude::*;

// 	#[test]
// 	fn works() {
// 		let mac = quote::quote! {
// 			<div>{value}</div>
// 		};
// 		let parsed = RsxPartsTokens::from_macro(mac);

// 		println!("{:#?}", parsed);
// 	}
// }
