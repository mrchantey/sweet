pub mod fs;
pub mod process;
pub mod terminal;

pub mod prelude {
	pub use crate::fs::*;
	pub use crate::process::*;
	pub use crate::terminal;
}
