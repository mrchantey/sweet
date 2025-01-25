use proc_macro2::TokenStream;

/// Instructions for converting the rust parts of an rsx node to tokens
/// see [RsxNodeTokens]
pub trait RsxRustTokens {
	/// not currently used but may be in the future
	fn ident() -> TokenStream;
	/// must return a valid RsxNode, ie RsxNode::TextBlock(some_string)
	fn map_node_block(block: &TokenStream) -> TokenStream;
	/// This should return an [RsxAttribute::Block](crate::prelude::RsxAttribute::Block)
	/// but can technically be any valid RsxAttribute or comma seperated list of RsxAttributes
	fn map_attribute_block(block: &TokenStream) -> TokenStream;
	/// This should return an [RsxAttribute::BlockValue](crate::prelude::RsxAttribute::BlockValue)
	/// but can technically be any valid RsxAttribute or comma seperated list of RsxAttributes
	fn map_attribute_value(key: &str, value: &TokenStream) -> TokenStream;
}
