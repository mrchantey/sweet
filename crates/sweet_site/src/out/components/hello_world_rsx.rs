use crate::prelude::*;
use anyhow::Result;
use forky::prelude::ReadFile;
use std::path::Path;
use sweet_rsx::prelude::*;



impl IntoHtml for HelloWorld {
	fn into_html() -> ParseResult<String> { Self::read_html_file(file!()) }
}


impl HydrateClient for HelloWorld {
	fn hydrate() -> ParseResult<()> {
		let html = Self::into_html()?;

		Ok(())
	}
}
