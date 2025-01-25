#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet_test::test_runner))]

pub mod error;
#[cfg(feature = "flume")]
pub mod flume_rsx;
pub mod rsx;
pub mod signals_rsx;
pub mod string_rsx;
#[cfg(feature = "tokens")]
pub mod tokens;
pub mod tree;
pub mod renderer;


#[rustfmt::skip]
pub mod prelude {
	pub use crate::renderer::*;
	pub use crate::error::*;
	pub use crate::tree::*;
	pub use crate::rsx::*;
	#[cfg(feature = "tokens")]
	pub use crate::tokens::*;

	// testing 
	#[cfg(test)]
	pub use crate as sweet;
	#[cfg(test)]
	pub use sweet_test::prelude::*;
}
