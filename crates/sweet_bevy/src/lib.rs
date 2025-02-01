#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet_test::test_runner))]

pub mod extensions;
pub mod resources;
pub mod systems;


pub mod prelude {
	pub use crate::extensions::*;
	pub use crate::resources::*;
	pub use crate::systems::*;
	#[cfg(feature = "rand")]
	pub use rand::Rng;
}
