use proc_macro::TokenStream;
mod rsx;

/// Make like a cool rsx thing
#[proc_macro]
pub fn rsx(tokens: TokenStream) -> TokenStream { rsx::RsxMacro::parse(tokens) }
