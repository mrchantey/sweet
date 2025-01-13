use super::ParseResult;
use forky::prelude::*;

pub trait IntoHtml {
	#[cfg(not(target_arch = "wasm32"))]
	fn read_html_file(file: &str) -> ParseResult<String> {
		use std::path::Path;
		let path = Path::new(file);
		let stem = path.file_stem().ok_or_else(|| "No file stem")?;

		let html_path = path.join(stem).with_extension("html");
		let html = ReadFile::to_string(html_path)?;
		Ok(html)
	}
	fn into_html() -> ParseResult<String>;
}
