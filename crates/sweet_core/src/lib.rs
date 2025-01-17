#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet_test::test_runner))]

pub mod error;
// nice and handy for commonly used tasks
pub mod tree;
pub mod utils;
pub use utils::log;
pub use utils::sleep::*;
pub mod rsx;

#[rustfmt::skip]
pub mod prelude {
	pub use crate::error::*;
	pub use crate::tree::*;
	pub use crate::utils::log::*;
	pub use crate::utils::sleep::*;
	pub use crate::utils::*;
	pub use crate::rsx::*;

	// testing 
	#[cfg(test)]
	pub use crate as sweet;
	#[cfg(test)]
	pub use sweet_rsx_macros::rsx;
	#[cfg(test)]
	pub use sweet_test::prelude::*;

}
