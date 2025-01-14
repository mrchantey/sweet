use sweet_core::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;



pub struct LiveBlock {
	pub el: HtmlElement,
	pub parts: Vec<String>,
}
impl LiveBlock {
	pub fn new(block: &HydratedBlock) -> ParseResult<Self> {
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
		let el = el.dyn_into::<HtmlElement>().unwrap();
		let parts = Self::parse_parts(&el)?;

		Ok(Self { el, parts })
	}

	/// for a given element with the attribute `s:block`, split the inner text into parts.
	/// We dont currently handle child nodes
	/// If there is no attribute there is only one part.
	/// example: <div s:id="0" s:block="4,6,18">the 11th value is 11</div>
	fn parse_parts(el: &HtmlElement) -> ParseResult<Vec<String>> {
		let block = el
			.get_attribute("data-sblock")
			.unwrap_or_else(|| "".to_string());
		if el.child_element_count() != 0 {
			return Err(ParseError::Hydration(
				"block elements with children is not yet supported".to_string(),
			));
		}

		let indices = block
			.split(",")
			.map(|s| s.parse::<usize>().unwrap())
			.collect::<Vec<usize>>();

		let inner_text = el.inner_text();
		let mut parts = Vec::new();
		let mut start = 0;
		for i in indices {
			parts.push(inner_text[start..i].to_string());
			start = i;
		}
		parts.push(inner_text[start..].to_string());
		Ok(parts)
	}
}
