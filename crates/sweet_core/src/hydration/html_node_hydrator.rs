use crate::prelude::*;

/// An implementation of hydrated that simply updates a tree of
/// html nodes
pub struct HtmlNodeHydrator {
	pub items: Vec<HtmlNode>,
	constants: HtmlConstants,
}

impl HtmlNodeHydrator {
	pub fn new(items: Vec<HtmlNode>, constants: HtmlConstants) -> Self {
		Self { items, constants }
	}
}

impl Hydrator for HtmlNodeHydrator {
	fn render(&self) -> String { self.items.render() }

	fn update_rsx_node(
		&mut self,
		rsx: RsxNode,
		cx: &RsxContext,
	) -> Result<(), HydrationError> {
		let id = cx.element_id.to_string();

		for html in self.items.iter_mut() {
			if let Some(el) = html
				.query_selector_attr(self.constants.id_attribute_key, Some(&id))
			{
				return apply_rsx(el, rsx, cx, &self.constants);
			} else {
			}
		}

		return Err(HydrationError::InvalidContext(format!(
			"Could not find node with id: {}",
			id
		)));
	}
}


/// we've found a html node with a matching id
#[allow(unused)]
fn apply_rsx(
	el: &mut HtmlElementNode,
	rsx: RsxNode,
	cx: &RsxContext,
	constants: &HtmlConstants,
) -> Result<(), HydrationError> {
	match rsx {
		RsxNode::Fragment(vec) => todo!(),
		RsxNode::Block {
			initial,
			register_effect,
		} => todo!(),
		RsxNode::Doctype => todo!(),
		RsxNode::Comment(_) => todo!(),
		RsxNode::Text(text) => {
			let child =
				el.children.get_mut(cx.element_child_index).ok_or_else(
					|| HydrationError::invalid_element("Could not find child"),
				)?;
			*child = HtmlNode::Text(text);

			// let text = text.
			// let attr = el
			// 	.get_attribute_value(&constants.block_attribute_key)
			// 	.ok_or_else(|| {
			// 	HydrationError::invalid_element(
			// 		"Could not find block attribute",
			// 	)
			// })?;
			// let child_positions =
			// 	TextBlockEncoder::decode(attr).map_err(|_| {
			// 		HydrationError::InvalidElement(format!(
			// 			"Could not decode block attribute: {}",
			// 			attr
			// 		))
			// 	})?;

			// println!("child_positions: {:?}", child_positions);
		}
		RsxNode::Element(rsx_element) => todo!(),
	}
	Ok(())
}
