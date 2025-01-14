#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet_test::test_runner))]
pub mod bench;
pub mod rsx;
pub mod test_runners;

pub mod prelude {
	pub use crate::bench::*;
	pub use crate::rsx::*;
	pub use crate::test_runners::*;
}
