//! Declarative full-stack test framework.
//!
//! # Usage
//!
//! ```rust
//! // examples/sweet.rs
//! # use sweet::prelude::*;
//! #[sweet_test]
//! fn it_passes() -> Result<()>{
//! 	expect(true).to_be_true()
//! }
//!
//! ```
//!
//! ```sh
//! cargo run --example sweet
//! ```
//!

#![feature(async_closure, doc_cfg)]
#![allow(async_fn_in_trait)]

pub use sweet_macros::sweet_test;

pub mod common;
// #[doc(hidden)]
/// Matchers used for assertions: `expect(true).to_be_true()`
pub mod matchers;
#[doc(inline)]
pub use matchers::MatcherExtClose;
/// Test case module
pub mod test_case;
/// Test runner module
pub mod test_runner;
/// Test suite module
pub mod test_suite;

#[cfg(not(target_arch = "wasm32"))]
#[doc(hidden)]
pub mod native;
#[cfg(target_arch = "wasm32")]
pub mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::visit;
#[cfg(target_arch = "wasm32")]
pub use wasm::MatcherHtml;
#[cfg(feature = "bevy")]
pub mod bevy_matchers;

pub mod prelude {
	#[cfg(feature = "bevy")]
	pub use crate::bevy_matchers::*;
	pub use crate::common::*;
	pub use crate::matchers::*;
	pub use crate::native::*;
	pub use crate::test_case::*;
	pub use crate::test_runner::*;
	pub use crate::test_suite::*;
	#[cfg(target_arch = "wasm32")]
	pub use crate::wasm::*;
	pub use anyhow::Result;
	pub use sweet_macros::sweet_test;
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

#[cfg(target_arch = "wasm32")]
pub fn main() -> anyhow::Result<()> { wasm::sweet_wasm_entry() }

/// Entry point for Sweet to run all automatically collected tests.
#[cfg(not(target_arch = "wasm32"))]
// #[tokio::main]
pub fn main() -> anyhow::Result<()> {
	use forky::prelude::*;
	native::RunTestsNativeCommand.run_with_cli_args()
}
