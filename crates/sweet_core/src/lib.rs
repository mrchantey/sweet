#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet_test::test_runner))]

pub mod error;
pub mod rsx;
pub mod tree;

#[rustfmt::skip]
pub mod prelude {
	pub use crate::error::*;
	pub use crate::tree::*;
	pub use crate::rsx::*;

	// testing 
	#[cfg(test)]
	pub use crate as sweet;
	#[cfg(test)]
	pub use sweet_test::prelude::*;
}
