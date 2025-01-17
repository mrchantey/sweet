#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet_test::test_runner))]

pub mod error;
// nice and handy for commonly used tasks
pub mod tree;
pub mod utils;
pub use utils::log::*;
pub use utils::sleep::*;

#[cfg(feature = "quote")]
#[path = "tree/_rsx_tree_quote.rs"]
pub mod _rsx_tree_quote;


#[rustfmt::skip]
pub mod prelude {
	pub use crate::error::*;
	#[cfg(feature = "quote")]
	#[allow(unused_imports)]
	pub use crate::_rsx_tree_quote::*;
	pub use crate::tree::*;
	pub use crate::utils::log::*;
	pub use crate::utils::sleep::*;
	pub use crate::utils::*;

	// testing 
	#[cfg(test)]
	pub use crate as sweet;
	#[cfg(test)]
	pub use sweet_rsx_macros::rsx;
	#[cfg(test)]
	pub use sweet_test::prelude::*;

}
