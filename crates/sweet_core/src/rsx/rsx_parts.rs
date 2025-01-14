use crate::prelude::*;
use std::path::PathBuf;


/// Information for a particular rsx! macro.
/// This is not the final parse step, we need another pass to
/// build the component tree and reassign attributes.
// #[derive(Debug)]
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
}


#[derive(Debug, Clone)]
pub enum PathOrInline {
	Path(PathBuf),
	Inline(String),
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
