use anyhow::Result;
use forky::prelude::ReadFile;
use std::path::Path;


pub fn render_html() -> Result<String> {
	let path = Path::new(file!())
		.parent()
		.unwrap_or(Path::new(""))
		.join("Counter.html");
	let html = ReadFile::to_string(path)?;
	Ok(html)
}
