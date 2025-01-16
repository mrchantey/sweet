#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet_test::test_runner))]



// easy re-export in sweet main crate
pub mod parse_rsx;

pub mod prelude {
	pub use crate::parse_rsx::*;
}
