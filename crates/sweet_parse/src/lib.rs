#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]


#[cfg(feature = "rsx")]
pub mod parse_rsx;
pub mod render;

pub mod prelude {
	#[cfg(feature = "rsx")]
	pub use crate::parse_rsx::*;
	pub use crate::render::*;
}

