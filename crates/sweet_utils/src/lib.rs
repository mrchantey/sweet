pub use utils::logging::*;
pub use utils::sleep::*;
pub mod extensions;
pub mod utils;


pub mod prelude {
	pub use crate::extensions::*;
	pub use crate::utils::*;
	#[cfg(feature = "rand")]
	pub use rand::Rng;
}
// #[cfg(test)]
// mod test {
// 	use crate::prelude::*;

// 	#[test]
// 	fn works() {
// 		assert_eq!(1, 0);
// 	}

// }
