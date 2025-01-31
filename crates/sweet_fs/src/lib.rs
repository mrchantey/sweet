#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet_test::test_runner))]
pub mod fs;
pub mod process;
pub mod terminal;

pub mod prelude {
	pub use crate::fs::*;
	pub use crate::process::*;
	pub use crate::terminal;
}
