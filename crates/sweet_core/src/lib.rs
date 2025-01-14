#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]

pub mod error;
// nice and handy for commonly used tasks
pub mod rsx;
pub mod utils;
pub use utils::log::*;
pub use utils::sleep::*;

pub mod prelude {
	pub use crate::error::*;
	pub use crate::rsx::*;
	pub use crate::utils::log::*;
	pub use crate::utils::sleep::*;
	pub use crate::utils::*;

}