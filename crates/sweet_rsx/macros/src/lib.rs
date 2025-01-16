use proc_macro::TokenStream;
mod rsx;

/// This macro expands to an [RsxTree<RustParts>](sweet_core::prelude::RsxTree)
/// ```
/// # use sweet::prelude::*;
/// let tree = rsx! {<div> the value is {3}</div>};
/// assert_eq!(tree.nodes.len(), 1);
///
/// ```
///
#[proc_macro]
pub fn rsx(tokens: TokenStream) -> TokenStream { rsx::RsxMacro::parse(tokens) }
