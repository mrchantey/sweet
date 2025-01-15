use proc_macro::TokenStream;
mod rsx;

/// This macro replaces its html contents with
/// [RsxParts](sweet_core::prelude::RsxParts).
/// ```
/// # use sweet::prelude::*;
/// let parts: RsxParts = rsx! {<div> the value is {3}</div>};
/// assert_eq!(parts.rust.len(), 1);
/// assert_eq!(parts.html.load().unwrap().to_string_placeholder(), "<div> the value is §</div>");
///
/// ```
///
#[proc_macro]
pub fn rsx(tokens: TokenStream) -> TokenStream { rsx::RsxMacro::parse(tokens) }
