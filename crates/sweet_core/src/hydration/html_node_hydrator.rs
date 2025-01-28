use crate::prelude::*;

/// An implementation of hydrated that simply updates a tree of
/// html nodes
pub struct HtmlNodeHydrator {
	pub html: Vec<HtmlNode>,
	constants: HtmlConstants,
	rust_node_map: RustNodeMap,
}

impl HtmlNodeHydrator {
	pub fn new(rsx: impl Rsx, constants: HtmlConstants) -> Self {
		let rsx = rsx.into_rsx();
		let html = RsxToHtml {
			resumable: true,
			..Default::default()
		}
		.map_node(&rsx);

		let rust_node_map = RustNodeMap::from_node(&rsx);

		Self {
			html,
			constants,
			rust_node_map,
		}
	}
}

impl Hydrator for HtmlNodeHydrator {
	fn render(&self) -> String { self.html.render() }

	fn update_rsx_node(
		&mut self,
		rsx: RsxNode,
		cx: &RsxContext,
	) -> Result<(), HydrationError> {
		let id = self
			.rust_node_map
			.rust_blocks
			.get(cx.rust_node_index())
			.ok_or_else(|| {
				HydrationError::InvalidContext(format!(
					"Could not find block parent for index: {}",
					cx.rust_node_index()
				))
			})?
			.to_string();

		for html in self.html.iter_mut() {
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
				el.children.get_mut(cx.child_index()).ok_or_else(|| {
					HydrationError::invalid_element("Could not find child")
				})?;
			*child = HtmlNode::Text(text);
		}
		RsxNode::Element(rsx_element) => todo!(),
	}
	Ok(())
}
