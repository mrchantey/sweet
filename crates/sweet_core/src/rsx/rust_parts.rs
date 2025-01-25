use crate::prelude::*;
use std::path::PathBuf;
use strum_macros::AsRefStr;

/// The rusty parts of a rsx tree.
#[derive(AsRefStr)]
pub enum RustParts {
	/// **rust**
	/// ie `<div>{value}</div>`
	TextBlock(String),
	/// ie `<div {attr_block}></div>`
	AttributeBlock2(String),
	/// ie `<div class={class_name}></div>`
	AttributeValue(String),
	/// ie `<div onclick={handle_click}></div>`,
	/// or the shorthand `<div on_click></div>`
	Event(HydratedEvent),
	/// ie `<div><Counter/></div>`
	Component(RsxTree<RustParts>),
}



impl RsxRust for RustParts {
	type NodeBlock = String;
	type AttributeBlock = String;
	type AttributeBlockValue = String;

	fn block_to_string(block: &Self::NodeBlock) -> String { block.clone() }
	fn attribute_block_to_string(block: &Self::AttributeBlock) -> String {
		block.clone()
	}
	fn attribute_block_value_to_string(
		block: &Self::AttributeBlockValue,
	) -> String {
		block.clone()
	}
}




impl std::fmt::Debug for RustParts {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct(self.as_ref()).finish()
	}
}

/// Either provide data in a file or stored as a string,
/// used for css and html.
/// Defaults to Inline(String::default())
pub enum PathOrInline<T> {
	Path(PathBuf),
	Inline(T),
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize + serde::de::DeserializeOwned> PathOrInline<T> {
	// pub fn to_bytes(self)

	pub fn load(self) -> ParseResult<T> {
		match self {
			#[allow(unused)]
			PathOrInline::Path(path) => {
				#[cfg(not(target_arch = "wasm32"))]
				{
					let bytes = forky::prelude::ReadFile::to_bytes(path)?;
					let value = bincode::deserialize(&bytes)?;

					return Ok(value);
				}
				#[cfg(target_arch = "wasm32")]
				todo!(
					"js_runtime from sweet_core, allow for no refresh reload"
				);
			}
			PathOrInline::Inline(value) => Ok(value),
		}
	}
}

impl<T: Default> Default for PathOrInline<T> {
	fn default() -> Self { PathOrInline::Inline(T::default()) }
}

impl<T: std::fmt::Debug> std::fmt::Debug for PathOrInline<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			PathOrInline::Path(path) => {
				f.debug_tuple("Path").field(path).finish()
			}
			PathOrInline::Inline(value) => {
				f.debug_tuple("Inline").field(value).finish()
			}
		}
	}
}
