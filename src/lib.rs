#[cfg(feature = "fs")]
pub use sweet_fs as fs;
#[cfg(feature = "test")]
pub use sweet_test as test;
#[cfg(feature = "test")]
pub use sweet_test::sweet_test_macros::*;
#[cfg(feature = "test")]
pub use sweet_test::test_runner;
pub use sweet_utils as utils;
pub use sweet_utils::elog;
pub use sweet_utils::log;
pub use sweet_utils::noop;
#[cfg(feature = "web")]
pub use sweet_web as web;

pub mod prelude {
	#[cfg(feature = "fs")]
	pub use sweet_fs::prelude::*;
	#[cfg(feature = "test")]
	pub use sweet_test::prelude::*;
	pub use sweet_utils::prelude::*;
	#[cfg(feature = "web")]
	pub use sweet_web::prelude::*;
}
