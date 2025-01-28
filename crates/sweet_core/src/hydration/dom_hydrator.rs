use super::rsx_context_map::RsxContextMap;
use crate::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;
use web_sys::Document;
use web_sys::Element;

/// A hydrator for working with the dom
pub struct DomHydrator {
	constants: HtmlConstants,
	// cache document reference
	document: Document,
	/// sparse set element array, cached for fast reference
	elements: Vec<Option<Element>>,
	rust_node_map: RsxContextMap,
}

impl Default for DomHydrator {
	fn default() -> Self {
		Self {
			constants: Default::default(),
			document: window().unwrap().document().unwrap(),
			elements: Default::default(),
			rust_node_map: Default::default(),
		}
	}
}

impl DomHydrator {
	/// we've found a html node with a matching id
	#[allow(unused)]
	fn apply_rsx(
		&self,
		el: Element,
		rsx: RsxNode,
		cx: &RsxContext,
	) -> Result<(), HydrationError> {
		Ok(())
	}

	/// try to get cached element or find it in the dom.
	/// This also uncollapses the child text nodes
	fn get_or_find_element(
		&mut self,
		cx: &RsxContext,
	) -> Result<Element, HydrationError> {
		if let Some(Some(el)) = self.elements.get(cx.html_element_index()) {
			return Ok(el.clone());
		}

		let query = format!(
			"[{}='{}']",
			self.constants.id_attribute_key,
			cx.html_element_index()
		);
		let rsx_id = cx.rust_node_index();

		if let Some(el) = self.document.query_selector(&query).unwrap() {
			self.elements.resize(rsx_id + 1, None);
			self.elements[rsx_id] = Some(el.clone());
			self.uncollapse_children(&el, rsx_id)?;
			Ok(el)
		} else {
			Err(HydrationError::InvalidContext(format!(
				"Could not find node with id: {}",
				rsx_id
			)))
		}
	}

	fn uncollapse_children(
		&self,
		el: &Element,
		rsx_id: usize,
	) -> Result<(), HydrationError> {
		if let Some(encoded) =
			el.get_attribute(self.constants.block_attribute_key)
		{
			let positions =
				TextBlockEncoder::decode(&encoded).map_err(|err| {
					HydrationError::InvalidContext(format!(
						"Could not decode block attribute on element: {}\n{:?}",
						rsx_id, err
					))
				})?;

			let indices = TextBlockPosition::into_split_positions(positions);

			let children = el.child_nodes();

			for (child_index, positions) in indices.into_iter().enumerate() {
				let child =
					children.item(child_index as u32).ok_or_else(|| {
						HydrationError::InvalidContext(format!(
							"Could not find child at index: {}",
							child_index
						))
					})?;
				let child: web_sys::Text = child.dyn_into().map_err(|_| {
					HydrationError::InvalidContext(format!(
						"Could not convert child to text node"
					))
				})?;
				// let mut cursor = 0;
				// let text = child.text
				// for position in positions {
				// 	let next_split = position - cursor;
				// 	child.split_text(next_split as u32 - cursor).unwrap();
				// 	cursor = position as u32;
				// 	sweet_utils::log!("child: {:?}", child);
				// }
			}
		}
		todo!();

		// Ok(())
	}
}


impl Hydrator for DomHydrator {
	/// returns body inner html
	fn render(&self) -> String {
		window()
			.unwrap()
			.document()
			.unwrap()
			.body()
			.unwrap()
			.inner_html()
	}

	fn update_rsx_node(
		&mut self,
		rsx: RsxNode,
		cx: &RsxContext,
	) -> Result<(), HydrationError> {
		let el = self.get_or_find_element(cx)?;
		match rsx {
			RsxNode::Block {
				initial,
				register_effect,
			} => {
				sweet_utils::log!("element found! {}", el.tag_name());
			}
			RsxNode::Text(val) => {
				if let Some(encoded) =
					el.get_attribute(self.constants.block_attribute_key)
				{
					let parts =
						TextBlockEncoder::decode(&encoded).map_err(|err| {
							HydrationError::InvalidContext(format!(
								"Could not decode block attribute on element: {}\n{:?}",
								cx,err
							))
						})?;
				} else {
					return Err(HydrationError::InvalidContext(format!(
						"Could not find block attribute on element: {}",
						cx
					)));
				};

				// let block_encoding = self.document.create_text_node(&val);
			}
			RsxNode::Fragment(vec) => todo!(),
			RsxNode::Doctype => todo!(),
			RsxNode::Comment(_) => todo!(),
			RsxNode::Element(rsx_element) => todo!(),
		}


		Ok(())
	}
}
