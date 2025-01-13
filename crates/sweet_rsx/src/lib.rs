#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]

pub mod rsx;
#[cfg(target_arch = "wasm32")]
pub mod sweet_loader;
// easy re-export in sweet main crate
pub use sweet_rsx_macros;
pub use sweet_rsx_macros::*;

pub mod prelude {
	pub use crate::rsx::*;
	#[cfg(target_arch = "wasm32")]
	pub use crate::sweet_loader::*;
	pub use sweet_rsx_macros::*;
}
