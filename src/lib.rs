#![cfg_attr(any(test, feature = "collect_libtest"), feature(test))]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(crate::test_runner))]
// #![cfg_attr(
// 	feature = "collect_libtest",
// 	test_runner(libtest_runner::libtest_runner)
// )]
#![feature(async_closure, doc_cfg, panic_payload_as_str)]
#![allow(async_fn_in_trait)]

//! Declarative full-stack test framework.
//!
//! # Usage
//!
//! ```rust
//! #![cfg_attr(test, feature(test, custom_test_frameworks))]
//! #![cfg_attr(test, test_runner(sweet::test_runner))]
//!
//!
//! # use sweet::prelude::*;
//! #[test]
//! fn it_passes() -> Result<()>{
//! 	assert!("this is a regular test");
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

pub mod common;
pub mod utils;
// #[doc(hidden)]
/// Matchers used for assertions: `expect(true).to_be_true()`
pub mod matchers;
#[doc(inline)]
pub use matchers::MatcherExtClose;
/// Utilities for [libtest](https://github.com/rust-lang/rust/tree/master/library/test)
pub mod libtest;
/// Test case module
pub mod test_case;
/// Test runner module
pub mod test_runner_utils;
/// Test suite module
pub mod test_suite;

#[cfg(not(target_arch = "wasm32"))]
#[doc(hidden)]
pub mod native;
#[cfg(target_arch = "wasm32")]
pub mod wasm_matchers;
#[cfg(target_arch = "wasm32")]
pub mod wasm_runner;
// #[cfg(target_arch = "wasm32")]
// pub use wasm::visit;
// #[cfg(target_arch = "wasm32")]
// pub use wasm::MatcherHtml;
#[cfg(feature = "bevy")]
pub mod bevy_matchers;

pub mod prelude {
	#[cfg(feature = "bevy")]
	pub use crate::bevy_matchers::*;
	pub use crate::common::*;
	pub use crate::libtest::*;
	pub use crate::matchers::*;
	#[cfg(not(target_arch = "wasm32"))]
	pub use crate::native::*;
	pub use crate::test_case::*;
	pub use crate::test_runner_utils::*;
	pub use crate::test_suite::*;
	pub use crate::utils::*;
	#[cfg(target_arch = "wasm32")]
	pub use crate::wasm_matchers::*;
	#[cfg(target_arch = "wasm32")]
	pub use crate::wasm_runner::*;
	pub use anyhow::Result;
}

/// Re-exports for macros
pub mod exports {
	// pub use crate::test_case::TestCaseConfig;

	pub use anyhow::Result;
	#[cfg(not(target_arch = "wasm32"))]
	pub use futures::future::CatchUnwind;
	#[cfg(not(target_arch = "wasm32"))]
	pub use futures::FutureExt;
	pub use inventory;
	// #[cfg(target_arch = "wasm32")]
	// pub use js_sys;
	//is full exports like this bad form?
	#[cfg(target_arch = "wasm32")]
	pub use js_sys::*;
	pub use serde_json;
	#[cfg(target_arch = "wasm32")]
	pub use wasm_bindgen;
	#[cfg(target_arch = "wasm32")]
	pub use wasm_bindgen::prelude::*;
	#[cfg(target_arch = "wasm32")]
	pub use wasm_bindgen_futures::future_to_promise;
}

#[cfg(any(test, feature = "collect_libtest"))]
pub fn test_runner(tests: &[&test::TestDescAndFn]) {
	#[cfg(target_arch = "wasm32")]
	crate::wasm_runner::run_libtest(tests);
	#[cfg(not(target_arch = "wasm32"))]
	crate::native::run_libtest(tests);
}

/// Entry point for Sweet to run inventory::collect tests.
#[cfg(not(target_arch = "wasm32"))]
// #[tokio::main]
pub fn main() -> anyhow::Result<()> {
	use forky::prelude::*;
	native::RunTestsNativeCommand.run_with_cli_args()
}
