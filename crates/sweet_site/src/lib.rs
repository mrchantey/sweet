#![allow(unused)]

pub mod components;
pub mod out;
pub mod pages;

pub mod prelude {
	pub use crate::components::*;
	pub use crate::out::*;
	pub use crate::pages::*;
}
