// // use super::*;
// use super::parse_test_case;
// use super::TestCaseFlags;
// use proc_macro2::Literal;
// use proc_macro2::TokenStream;
// use quote::quote;
// use syn::ItemFn;
// use syn::ReturnType;

// pub struct TestCaseAttr2 {
// 	// pub out: TokenStream,
// }

// impl TestCaseAttr2 {
// 	pub fn parse(
// 		_attr: proc_macro::TokenStream,
// 		input: proc_macro::TokenStream,
// 	) -> syn::Result<TokenStream> {
// 		let func = syn::parse::<ItemFn>(input)?;

//         let is_async = func.sig.asyncness.is_some();


// 		// let submit = parse_test_case(&wrapped, &flags);
// 		let out = quote! {
// 			#[test]
// 			#func
// 		}
// 		.into();
// 		Ok(out)
// 	}
// }
