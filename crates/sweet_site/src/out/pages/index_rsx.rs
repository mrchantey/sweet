use crate::pages::Index;
use crate::prelude::*;
use anyhow::Result;
use forky::prelude::*;
use sweet_rsx::prelude::*;
use sweet_core::prelude::*;


impl IntoHtml for Index {
	fn into_html() -> ParseResult<String> {
		let html = Self::read_html_file(file!())?;

		let counter = Counter::into_html()?;

		let html = html.replace("{{1}}", &counter);

		Ok(html)
	}
}
