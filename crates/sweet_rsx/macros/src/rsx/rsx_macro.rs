// use super::collect_nodes;
// use super::empty_elements;
// use super::parse_nodes;
// use super::WalkNodes;

use quote::ToTokens;
use sweet_parse::prelude::RsxPlugin;
use sweet_parse::prelude::SweetRsxPlugin;

pub struct RsxMacro;


impl RsxMacro {
	pub fn parse(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
		let tokens = syn::parse_macro_input!(tokens as syn::ItemMacro);
		SweetRsxPlugin::new_with_errors()
			.parse_tokens(tokens.to_token_stream())
			.map(|(tokens, _)| tokens)
			.unwrap_or_else(syn::Error::into_compile_error)
			.into()
	}
}
