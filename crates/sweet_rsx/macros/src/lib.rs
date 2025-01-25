use proc_macro::TokenStream;

/// This macro expands to an [RsxNode](sweet_core::prelude::RsxNode).
///
/// The type of node is determied by the feature flags, current options are:
/// - [`StringRsx`](sweet_core::rsx::StringRsx)
/// ```
/// # use sweet::prelude::*;
/// let tree = rsx! {<div> the value is {3}</div>};
/// assert_eq!(tree.nodes.len(), 1);
///
/// ```
///
#[proc_macro]
pub fn rsx(tokens: TokenStream) -> TokenStream { RsxMacro::parse(tokens) }


use sweet_core::string_rsx::StringRsx;
use sweet_rsx::prelude::RsxParser;
struct RsxMacro;


impl RsxMacro {
	pub fn parse(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
		let mut tokens: proc_macro2::TokenStream = tokens.into();
		let _output = RsxParser::<StringRsx>::default().parse_rsx(&mut tokens);
		// ignore output because errors are included in the token stream

		tokens.into()
	}
}
