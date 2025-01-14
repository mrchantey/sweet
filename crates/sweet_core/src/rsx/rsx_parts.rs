use crate::prelude::*;
use std::path::PathBuf;


/// Information for a particular rsx! macro.
/// This is not the final parse step, we need another pass to
/// build the component tree and reassign attributes.
// #[derive(Debug)]
pub struct RsxParts {
	pub events: Vec<HydratedEvent>,
	// the initial value of blocks
	pub blocks: Vec<String>,
	pub html: PathOrInline,
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
