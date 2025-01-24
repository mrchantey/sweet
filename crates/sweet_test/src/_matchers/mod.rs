pub mod common;
#[allow(unused_imports)]
pub use self::common::*;
#[cfg(target_arch = "wasm32")]
pub mod wasm_matchers;
#[cfg(target_arch = "wasm32")]
pub use self::wasm_matchers::*;
#[cfg(not(target_arch = "wasm32"))]
pub mod native_matchers;
#[cfg(not(target_arch = "wasm32"))]
pub use self::native_matchers::*;
