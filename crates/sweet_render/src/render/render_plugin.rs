use sweet_core::prelude::*;



pub trait RenderPlugin {
	fn render(self, rsx: impl Rsx) -> ParseResult<String>;
}
