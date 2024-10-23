mod macros;
use macros::*;
use proc_macro::TokenStream;

/// Mark a function to be ran by the sweet test runner.
///
/// # Accepted Signatures
/// ```rust
///
/// #[sweet_test]
/// fn empty() {}
///
/// #[sweet_test]
/// fn returns_result() -> sweet::Result<()> {}
///
/// #[sweet_test]
/// async fn is_async() {}
/// #[sweet_test(skip)]
/// #[ignore]
/// async fn is_async() {}
///
/// ```
///
///
/// # Attributes
/// - `#[sweet_test(skip)]`: Skips the test
/// - `#[sweet_test(only)]`: Skips all other tests in file
/// - `#[sweet_test(e2e)]`: Runs in-browser wasm tests in a seperate process as an iframe
/// - `#[sweet_test(non_send)]`: Always runs the test in the main thread which is required in crates like `bevy` and `fantoccini`.
///
#[proc_macro_attribute]
pub fn sweet_test(attr: TokenStream, input: TokenStream) -> TokenStream {
    TestCaseAttr::parse(attr, input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
