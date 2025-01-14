#![allow(unused)]

pub mod components;
#[cfg(feature = "render")]
pub mod render;
pub mod pages;

pub mod prelude {
	pub use crate::components::*;
	#[cfg(feature = "render")]
	pub use crate::render::*;
	pub use crate::pages::*;
}
