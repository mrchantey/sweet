use crate::prelude::*;
use std::path::PathBuf;

/// The rust, html and css extracted from an `rsx!` macro.
/// Note that the outputted html and css is not final,
/// it contains a placeholder that is filled in the render step.
#[derive(Default)]

pub struct RsxParts {
	pub rust: Vec<RsxRust>,
	pub html: PathOrInline,
	pub css: PathOrInline,
}

impl RsxParts {
	pub const DEFAULT_PLACEHOLDER: &'static str = "§";
	pub fn default_placeholder() -> String {
		Self::DEFAULT_PLACEHOLDER.to_string()
	}
}

/// The event or the indentifiers/blocks `ToString`.
pub enum RsxRust {
	/// Used internally for reconciling this
	/// element's other blocks, ie `<div data-sid="0"></div>`
	DynNodeId,
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
	ChildComponent(RsxParts),
}

impl std::fmt::Debug for RsxParts {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("RsxParts")
			.field("rust.len", &self.rust.len())
			.field("html", &self.html)
			.field("css", &self.css)
			.finish()
	}
}

/// Either provide data in a file or stored as a string,
/// used for css and html.
/// Defaults to Inline(String::default())
#[derive(Debug, Clone)]
pub enum PathOrInline {
	Path(PathBuf),
	Inline(String),
}

impl PathOrInline {
	pub fn load(self) -> ParseResult<String> {
		match self {
			PathOrInline::Path(path) => {
				#[cfg(not(target_arch = "wasm32"))]
				let html = forky::prelude::ReadFile::to_string(path)?;
				#[cfg(target_arch = "wasm32")]
				let html = String::new();
				#[cfg(target_arch = "wasm32")]
				todo!(
					"js_runtime from sweet_core, allow for no refresh reload"
				);
				Ok(html)
			}
			PathOrInline::Inline(html) => Ok(html),
		}
	}
}

impl Default for PathOrInline {
	fn default() -> Self { PathOrInline::Inline(String::default()) }
}

/// The parts were constructed by the preprocessor, with the
/// html living in a separate file.
pub struct PreprocessedRsxParts {
	pub html_path: PathBuf,
}
/// The parts were constructed by an unprocessed rsx! macro, with the
/// html living in the code.
pub struct InlineRsxParts {
	pub html: String,
	pub hydrate_item: Hydrated,
}
