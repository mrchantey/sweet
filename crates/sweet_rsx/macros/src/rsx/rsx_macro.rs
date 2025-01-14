// use super::collect_nodes;
// use super::empty_elements;
// use super::parse_nodes;
// use super::WalkNodes;
use sweet_parse::prelude::RsxPlugin;
use sweet_parse::prelude::SweetRsxPlugin;

pub struct RsxMacro;


impl RsxMacro {
	pub fn parse(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
		let mut tokens: proc_macro2::TokenStream = tokens.into();
		let result = SweetRsxPlugin::new_with_errors().parse_rsx(&mut tokens);

		if let Err(err) = result {
			let error = err.to_compile_error();
			let tokens = quote::quote! {
				#tokens
				#error
			};
			let tokens: proc_macro::TokenStream = tokens.into();
			return tokens;
		}

		let tokens: proc_macro::TokenStream = tokens.into();

		tokens
	}
}
