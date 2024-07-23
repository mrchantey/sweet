#![feature(async_closure)]
pub use sweet::*;
#[cfg(feature = "bevy")]
mod bevy_tests;
mod common;
#[cfg(target_arch = "wasm32")]
mod wasm_tests;
