use crate::pages::counter::Counter;
use anyhow::Result;
use forky::prelude::ReadFile;
use std::path::Path;
use sweet_core::prelude::*;
use sweet_rsx::prelude::*;

impl IntoHtml for Counter {
	fn into_html() -> ParseResult<String> { Self::read_html_file(file!()) }
}
