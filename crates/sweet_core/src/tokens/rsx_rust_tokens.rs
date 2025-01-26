use proc_macro2::TokenStream;
use quote::quote;

/// Instructions for converting the rust parts of an rsx node to tokens.
///
/// By default the ident is used to call specific transform methods,
/// if using this be sure to implement the following:
/// - map_node_block
/// - map_attribute_block
/// - map_attribute_value
/// - map_event
pub trait RsxRustTokens {
	fn ident() -> TokenStream;

	/// must return a valid RsxNode, ie RsxNode::Block
	fn map_node_block(block: &TokenStream) -> TokenStream {
		let ident = Self::ident();
		quote! {#ident::map_node_block(#block)}
	}

	/// This should return an [RsxAttribute::Block](crate::prelude::RsxAttribute::Block)
	/// but can technically be any valid RsxAttribute or comma seperated list of RsxAttributes
	fn map_attribute_block(block: &TokenStream) -> TokenStream {
		let ident = Self::ident();
		quote! {#ident::map_attribute_block(#block)}
	}

	/// This should return an [RsxAttribute::BlockValue](crate::prelude::RsxAttribute::BlockValue)
	/// but can technically be any valid RsxAttribute or comma seperated list of RsxAttributes
	fn map_attribute_value(key: &str, value: &TokenStream) -> TokenStream {
		let ident = Self::ident();
		quote! {#ident::map_attribute_value(#key, #value)}
	}
	/// This should return an [RsxAttribute::BlockValue](crate::prelude::RsxAttribute::BlockValue)
	/// but can technically be any valid RsxAttribute or comma seperated list of RsxAttributes
	fn map_event(key: &str, value: &TokenStream) -> TokenStream {
		let ident = Self::ident();
		quote! {#ident::map_event(#key, #value)}
	}
}
