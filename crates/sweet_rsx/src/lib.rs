pub use sweet_rsx_macros;
pub use sweet_rsx_macros::*;
pub mod rsx;

pub mod prelude {
	pub use crate::rsx::*;
	pub use sweet_rsx_macros::*;
}
