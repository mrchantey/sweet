use super::super::components::Counter;
use anyhow::Result;
use forky::prelude::*;


pub fn render_html() -> Result<String> {
	let file = file!();
	let path = std::path::Path::new(file!())
		.parent()
		.unwrap()
		.join("index.html");
	let html = ReadFile::to_string(path)?;

	let counter = Counter::render_html()?;

	let html = html.replace("{{1}}", &counter);

	Ok(html)
}
