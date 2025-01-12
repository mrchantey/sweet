#[cfg(feature = "rsx")]
pub use sweet_rsx as rsx;
#[cfg(feature = "rsx")]
pub use sweet_rsx::sweet_rsx_macros::*;
#[cfg(feature = "test")]
pub use sweet_test as test;
#[cfg(feature = "test")]
pub use sweet_test::sweet_test_macros::*;
#[cfg(feature = "test")]
pub use sweet_test::test_runner;


pub mod prelude {
	#[cfg(feature = "rsx")]
	pub use sweet_rsx::prelude::*;
	#[cfg(feature = "test")]
	pub use sweet_test::prelude::*;
}
