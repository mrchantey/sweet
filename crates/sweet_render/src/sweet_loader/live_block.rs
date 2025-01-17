use sweet_core::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;



pub struct LiveBlock {
	pub el: HtmlElement,
	pub parts: Vec<String>,
}
impl LiveBlock {
	pub fn new(block: &HydratedTextBlock) -> ParseResult<Self> {
		let document = web_sys::window().unwrap().document().unwrap();
		let el = document
			.query_selector(&format!("[data-sid=\"{}\"]", block.node_id))
			.ok()
			.flatten()
			.ok_or_else(|| {
				ParseError::Hydration(format!(
					"failed to find block element with id {}",
					block.node_id
				))
			})?;
		let _el = el.dyn_into::<HtmlElement>().unwrap();
		// let parts = Self::parse_parts(&el)?;
		todo!();
		// Ok(Self { el, parts })
	}
}
