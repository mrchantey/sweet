#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet_test::test_runner))]

pub mod error;
pub mod html;
pub mod hydration;
pub mod rsx;
pub mod signals_rsx;
pub mod string_rsx;
pub mod tree;
pub use sweet_rsx_macros::rsx;

#[rustfmt::skip]
pub mod prelude {
	pub use sweet_rsx_macros::rsx;
	pub use crate::hydration::*;
	pub use crate::error::*;
	pub use crate::html::*;
	pub use crate::tree::*;
	pub use crate::rsx::*;

	// testing 
	#[cfg(test)]
	pub use crate as sweet;
	#[cfg(test)]
	pub use sweet_test::prelude::*;
}
