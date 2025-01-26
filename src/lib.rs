#[cfg(feature = "core")]
pub use sweet_core as core;
#[cfg(feature = "render")]
pub use sweet_render as render;
#[cfg(feature = "rsx")]
pub use sweet_rsx as rsx;
#[cfg(feature = "macros")]
pub use sweet_rsx_macros::*;
#[cfg(feature = "server")]
pub use sweet_server as server;
#[cfg(feature = "test")]
pub use sweet_test as test;
#[cfg(feature = "test")]
pub use sweet_test::sweet_test_macros::*;
#[cfg(feature = "test")]
pub use sweet_test::test_runner;
pub use sweet_utils as utils;
pub use sweet_utils::noop;

pub mod prelude {
	#[cfg(feature = "core")]
	pub use sweet_core::prelude::*;
	#[cfg(feature = "rsx")]
	pub use sweet_rsx::prelude::*;
	#[cfg(feature = "macros")]
	pub use sweet_rsx_macros::rsx;
	#[cfg(feature = "server")]
	pub use sweet_server::prelude::*;
	#[cfg(feature = "test")]
	pub use sweet_test::prelude::*;
	#[cfg(feature = "render")]
	pub use sweet_utils::prelude::*;
}
