use crate::prelude::*;
/// Alias for [`Matcher::new()`]
///
/// # Example
///
/// ```rust
/// # use sweet_test::prelude::*;
/// expect(true).to_be_true();
/// expect("foobar").not().to_start_with("bar");
/// ```
pub fn expect<T>(value: T) -> Matcher<T> { Matcher::new(value) }
