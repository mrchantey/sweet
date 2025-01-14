use crate::prelude::*;
use std::path::PathBuf;


/// All information required to hydrate an rsx! macro.
#[derive(Debug)]
pub struct RsxParts {
	pub hydrate_item: Hydrated,
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
