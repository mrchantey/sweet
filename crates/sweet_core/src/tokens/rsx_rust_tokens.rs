use proc_macro2::TokenStream;

/// Instructions for converting the rust parts of an rsx node to tokens
/// see [RsxNodeTokens]
pub trait RsxRustTokens {
	fn ident() -> TokenStream;
	fn map_block(block: &TokenStream) -> TokenStream;
	fn map_attribute_block(block: &TokenStream) -> TokenStream;
	fn map_attribute_value(key: &str, value: &TokenStream) -> TokenStream;
}
