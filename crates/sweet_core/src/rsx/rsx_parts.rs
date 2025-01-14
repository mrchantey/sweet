use crate::prelude::*;
use std::path::PathBuf;

/// Information for a particular rsx! macro.
/// This is not the final parse step, we need another pass to
/// build the component tree and reassign attributes.
#[derive(Default)]
pub struct RsxParts {
	pub rust: Vec<RsxRust>,
	pub html: PathOrInline,
	pub css: PathOrInline,
}

/// The event or the indentifiers/blocks `ToString`.
pub enum RsxRust {
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
			.field("rust count", &self.rust.len())
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
