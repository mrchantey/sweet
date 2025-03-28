pub use utils::log::*;
pub use utils::sleep::*;
pub mod extensions;
pub mod path_utils;
pub mod utils;



pub mod prelude {
	pub use crate::canonical_file;
	pub use crate::extensions::*;
	pub use crate::path_utils::*;
	pub use crate::utils::*;
	#[cfg(feature = "rand")]
	pub use rand::Rng;
}


pub mod exports {
	pub use glob;
}
