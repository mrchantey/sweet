#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet_test::test_runner))]

pub mod error;
// nice and handy for commonly used tasks
pub mod rsx;
pub mod utils;
pub use utils::log::*;
pub use utils::sleep::*;

#[cfg(feature = "quote")]
#[path = "rsx/_html_partial_quote.rs"]
pub mod html_partial_quote;

pub mod prelude {
	pub use crate::error::*;
	#[cfg(feature = "quote")]
	#[allow(unused_imports)]
	pub use crate::html_partial_quote::*;
	pub use crate::rsx::*;
	pub use crate::utils::log::*;
	pub use crate::utils::sleep::*;
	pub use crate::utils::*;
}
