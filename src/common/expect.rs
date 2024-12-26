use crate::matchers::Matcher;
/// Alias for `Matcher::new()`
///
/// # Example
///
/// ```rust
/// # use sweet::prelude::*;
/// expect(true).to_be_true()?;
/// expect("foobar").not().to_start_with("bar")?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn expect<T>(value: T) -> Matcher<T> {
    Matcher::new(value)
}
