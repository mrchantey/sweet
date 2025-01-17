#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet_test::test_runner))]
#![feature(type_alias_impl_trait)]

pub mod render;
#[cfg(target_arch = "wasm32")]
pub mod sweet_loader;


#[rustfmt::skip]
pub mod prelude {
	pub use crate::render::*;
	#[cfg(target_arch = "wasm32")]
	pub use crate::sweet_loader::*;

	// testing
	#[cfg(test)]
	pub use sweet_core as sweet;
	#[cfg(test)]
	pub use sweet_core::prelude::*;
	#[cfg(test)]
	pub use sweet_rsx_macros::rsx;
	#[cfg(test)]
	pub use sweet_test::prelude::*;
}
