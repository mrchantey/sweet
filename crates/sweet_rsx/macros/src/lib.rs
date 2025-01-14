use proc_macro::TokenStream;
mod rsx;

/// This macro replaces its html contents with
/// [RsxParts](sweet_core::prelude::RsxParts).
///
/// ```
/// # use sweet::prelude::*;
/// let parts: RustParts = rsx! {<div> the value is {3} </div>};
/// assert_eq!(parts.rust_count(), 1);
/// assert_eq!(parts.html, "<div> the value is §</div>");
///
/// ```
///
#[proc_macro]
pub fn rsx(tokens: TokenStream) -> TokenStream { rsx::RsxMacro::parse(tokens) }
