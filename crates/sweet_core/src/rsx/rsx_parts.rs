use crate::prelude::*;
use std::collections::VecDeque;
use std::path::PathBuf;
use strum_macros::AsRefStr;

/// The rust, html and css extracted from an `rsx!` macro.
/// Note that the outputted html and css is not final,
/// it contains a placeholder that is filled in the render step.
#[derive(Default)]



pub struct RsxParts {
	/// The rust blocks extracted from the rsx! macro,
	/// collected via Depth First Search traversal.
	pub rust: VecDeque<RsxRust>,
	pub html: PathOrInline<HtmlPartial>,
}

impl RsxParts {}

impl std::fmt::Debug for RsxParts {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("RsxParts")
			.field(
				"rust",
				&self.rust.iter().map(|r| r.as_ref()).collect::<Vec<_>>(),
			)
			.field("html", &self.html)
			.finish()
	}
}

/// The event or the indentifiers/blocks `ToString`.
#[derive(AsRefStr)]
pub enum RsxRust {
	/// Any element containing rust needs a node id
	DynNodeId,
	/// **rust**
	/// ie `<div>{value}</div>`
	InnerText(String),
	/// ie `<div {attr_key}=true></div>`
	AttributeKey(String),
	/// ie `<div class={class_name}></div>`
	AttributeValue(String),
	/// ie `<div onclick={handle_click}></div>`,
	/// or the shorthand `<div on_click></div>`
	Event(HydratedEvent),
	/// ie `<div><Counter/></div>`
	Component(RsxParts),
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
