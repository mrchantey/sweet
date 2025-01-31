#[cfg(feature = "test")]
pub use sweet_test as test;
#[cfg(feature = "test")]
pub use sweet_test::sweet_test_macros::*;
#[cfg(feature = "test")]
pub use sweet_test::test_runner;
pub use sweet_utils as utils;
pub use sweet_utils::noop;

pub mod prelude {
	#[cfg(feature = "test")]
	pub use sweet_test::prelude::*;
}
