use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
mod rstml_demo;


#[proc_macro]
pub fn rsx(_tokens: TokenStream) -> TokenStream {
	quote! {}.to_token_stream().into()
	// view_macro_impl(tokens, false)
}


/// Converts HTML to `String`.
///
/// Values returned from braced blocks `{}` are expected to return something
/// that implements `Display`.
///
/// See [rstml docs](https://docs.rs/rstml/) for supported tags and syntax.
///
/// # Example
///
/// ```
/// use rstml_to_string_macro::html;
/// // using this macro, one should write docs module on top level of crate.
/// // Macro will link html tags to them.
/// pub mod docs {
///     /// Element has open and close tags, content and attributes.
///     pub fn element() {}
/// }
/// # fn main (){
///
/// let world = "planet";
/// assert_eq!(html!(<div>"hello "{world}</div>), "<div>hello planet</div>");
/// # }
/// ```
#[proc_macro]
pub fn html(tokens: TokenStream) -> TokenStream { rstml_demo::html_inner(tokens, false) }

/// Same as html but also emit IDE helper statements.
/// Open tests.rs in ide to see semantic highlight/goto def and docs.
#[proc_macro]
pub fn html_ide(tokens: TokenStream) -> TokenStream { rstml_demo::html_inner(tokens, true) }
