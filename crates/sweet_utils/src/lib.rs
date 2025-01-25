#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet_test::test_runner))]


pub use utils::log::*;
pub use utils::sleep::*;
pub mod utils;


pub mod prelude {
	pub use crate::utils::log::*;
	pub use crate::utils::sleep::*;

	pub use crate::utils::*;
}
