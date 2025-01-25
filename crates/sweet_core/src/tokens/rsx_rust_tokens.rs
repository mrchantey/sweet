use crate::prelude::*;
use proc_macro2::TokenStream;
use quote::quote;

/// Instructions for converting the rust parts of an rsx node to tokens
/// see [RsxNodeTokens]
pub trait RsxRustTokens {
	fn ident() -> syn::Ident;

	fn map_block(block: &TokenStream) -> TokenStream;
	fn map_attribute_block(block: &TokenStream) -> TokenStream;
	fn map_attribute_value(key: &str, value: &TokenStream) -> TokenStream;
	// {
	// 	match attr {
	// 		NodeAttribute::Block(block) => RsxAttributeTokens::Block(
	// 			quote! {RustParts::AttributeBlock(#block)},
	// 		),
	// 		NodeAttribute::Attribute(attr) => {
	// 			let key = attr.key.to_string();

	// 			if key.starts_with("on") {
	// 				let tokens = if let Some(value) = attr.value() {
	// 					value.to_token_stream()
	// 				} else {
	// 					// default to a function called onclick
	// 					Ident::new(&key, key.span()).to_token_stream()
	// 				};
	// 				RsxAttribute::BlockValue {
	// 					key,
	// 					value: quote! {RustParts::Event(Box::new(#tokens))},
	// 				}
	// 			} else if let Some(value) = attr.value() {
	// 				match value {
	// 					// only literals (string, number, bool) are not rusty
	// 					syn::Expr::Lit(expr_lit) => {
	// 						let value = match &expr_lit.lit {
	// 							syn::Lit::Str(s) => s.value(),
	// 							other => other.to_token_stream().to_string(),
	// 						};
	// 						RsxAttribute::KeyValue { key, value }
	// 					}
	// 					tokens => RsxAttribute::BlockValue {
	// 						key,
	// 						value: quote! {RustParts::AttributeValue(#tokens.to_string())},
	// 					},
	// 				}
	// 			} else {
	// 				RsxAttribute::Key { key }
	// 			}
	// 		}
	// }
	// fn map_event(key: &str, value: Option<TokenStream>) -> TokenStream;
}





impl RsxRustTokens for StringRsx {
	fn ident() -> syn::Ident {
		syn::Ident::new("StringRsx", proc_macro2::Span::call_site())
	}

	fn map_block(block: &TokenStream) -> TokenStream {
		quote! { #block.to_string() }
	}

	fn map_attribute_block(block: &TokenStream) -> TokenStream {
		quote! { #block.to_string() }
	}

	fn map_attribute_value(_key: &str, value: &TokenStream) -> TokenStream {
		quote! { #value.to_string() }
	}
}
