#[cfg(feature = "test")]
pub use sweet_test as test;
#[cfg(feature = "test")]
pub use sweet_test::test;
#[cfg(feature = "test")]
pub use sweet_test::test_runner;


pub mod prelude {
	#[cfg(feature = "test")]
	pub use sweet_test::prelude::*;
}
