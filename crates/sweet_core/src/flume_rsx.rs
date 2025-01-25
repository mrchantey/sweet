use crate::prelude::*;


/// A basic interactive rsx implementation
pub struct FlumeRsx;


#[cfg(feature = "tokens")]
use proc_macro2::TokenStream;
#[cfg(feature = "tokens")]
use quote::quote;

#[cfg(feature = "tokens")]
impl RsxRustTokens for FlumeRsx {
	fn ident() -> TokenStream {
		quote! {sweet::flume_rsx::FlumeRsx}
	}

	fn map_block(block: &TokenStream) -> TokenStream {
		quote! { #block.to_string() }
	}

	fn map_attribute_block(block: &TokenStream) -> TokenStream {
		quote! { #block.to_string() }
	}

	fn map_attribute_value(key: &str, value: &TokenStream) -> TokenStream {
		if key.starts_with("on") {
			let str = format!("{key}_handler");
			quote! { #str.to_string() }
		} else {
			quote! { #value.to_string() }
		}
	}
}
