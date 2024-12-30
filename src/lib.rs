#![cfg_attr(any(test, feature = "collect_libtest"), feature(test))]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(crate::test_runner))]
#![feature(panic_payload_as_str)]
//!
//! # Usage
//!
//! ```rust
//! #![cfg_attr(test, feature(test, custom_test_frameworks))]
//! #![cfg_attr(test, test_runner(sweet::test_runner))]
//!
//!
//! # use sweet::prelude::*;
//!
//! #[test]
//! fn it_passes() -> Result<()>{
//! 	assert!(1 + 1 == 2);
//! 	expect("sweet").not().to_contain("verbose matchers")
//! }
//!
//! ```
//!
//! ```sh
//! cargo run --example sweet
//! ```
//!
extern crate test;
// the #[sweet::test] macro
pub use sweet_macros::test;
// #[cfg(test)]
// use libtest_runner::testlib_runner as libtest_runner;
pub mod backtrace;
/// Utilities for [libtest](https://github.com/rust-lang/rust/tree/master/library/test)
pub mod libtest;
/// Test runner module
pub mod test_runner;
/// Test suite module
pub mod test_suite;
pub mod utils;

#[path = "_matchers/mod.rs"]
/// Matchers used for assertions: `expect(true).to_be_true()`
pub mod matchers;
#[cfg(not(target_arch = "wasm32"))]
#[doc(hidden)]
pub mod native;
pub mod sweet_test;
#[cfg(target_arch = "wasm32")]
pub mod wasm_runner;

pub mod prelude {
	pub use crate::backtrace::*;
	pub use crate::libtest::*;
	pub use crate::matchers::*;
	#[cfg(not(target_arch = "wasm32"))]
	pub use crate::native::*;
	pub use crate::sweet_test::*;
	pub use crate::test_runner::*;
	pub use crate::test_suite::*;
	pub use crate::utils::*;
	#[cfg(target_arch = "wasm32")]
	pub use crate::wasm_runner::*;
	pub use anyhow::Result;
}

pub fn test_runner(tests: &[&test::TestDescAndFn]) {
	#[cfg(target_arch = "wasm32")]
	crate::wasm_runner::run_libtest(tests);
	#[cfg(not(target_arch = "wasm32"))]
	crate::native::run_libtest(tests);
}
