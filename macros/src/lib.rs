mod macros;
use macros::*;
use proc_macro::TokenStream;

/// Mark a function to be ran by the sweet test runner.
///
/// # Accepted Signatures
/// ```rust
///
/// #[sweet::test]
/// fn empty() {}
///
/// #[sweet::test]
/// fn returns_result() -> sweet::Result<()> {}
///
/// #[sweet::test]
/// async fn is_async() {}
/// #[sweet::test(skip)]
/// #[ignore]
/// async fn is_async() {}
///
/// ```
///
///
/// # Attributes
/// - `#[sweet::test(skip)]`: Skips the test
/// - `#[sweet::test(only)]`: Skips all other tests in file
/// - `#[sweet::test(e2e)]`: Runs in-browser wasm tests in a seperate process as an iframe
/// - `#[sweet::test(non_send)]`: Always runs the test in the main thread which is required in crates like `bevy` and `fantoccini`.
///
#[proc_macro_attribute]
pub fn test(attr: TokenStream, input: TokenStream) -> TokenStream {
	TestCaseAttr::parse(attr, input)
		.unwrap_or_else(syn::Error::into_compile_error)
		.into()
}
