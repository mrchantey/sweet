mod html_node;
use crate::rsx::RsxContext;
use crate::rsx::RsxNode;
pub use html_node::*;


#[derive(Debug, thiserror::Error)]
pub enum HydrationError {
	#[error("Invalid context: {0}")]
	InvalidContext(String),
}

impl HydrationError {
	pub fn invalid_context(msg: &str) -> Self {
		HydrationError::InvalidContext(msg.to_string())
	}
}


pub trait Hydrated {
	fn update_rsx_node(
		&mut self,
		node: RsxNode,
		cx: &RsxContext,
	) -> Result<(), HydrationError>;
}

/// An implementation of hydrated that simply updates a tree of
/// html nodes
pub struct HydratedHtml {
	pub items: Vec<HtmlNode>,
	constants: HtmlConstants,
}

impl HydratedHtml {
	pub fn new(items: Vec<HtmlNode>, constants: HtmlConstants) -> Self {
		Self { items, constants }
	}
}

impl Hydrated for HydratedHtml {
	fn update_rsx_node(
		&mut self,
		rsx: RsxNode,
		cx: &RsxContext,
	) -> Result<(), HydrationError> {
		let id = cx.html_pos.to_csv();

		// a kind of simulated Document.querySelector
		for html in self.items.iter_mut() {
			if let Some(el) = html.query_selector_attr(
				self.constants.id_attribute_key,
				Some(id.as_str()),
			) {
				apply_rsx(el, rsx);
				return Ok(());
			} else {
			}
		}

		return Err(HydrationError::InvalidContext(format!(
			"Could not find node with id: {}",
			id
		)));
	}
}


/// we've found a html node with a matchinf
fn apply_rsx(el: &mut HtmlElementNode, rsx: RsxNode) {
	match rsx {
		RsxNode::Fragment(vec) => todo!(),
		RsxNode::Block {
			initial,
			register_effect,
		} => todo!(),
		RsxNode::Doctype => todo!(),
		RsxNode::Comment(_) => todo!(),
		RsxNode::Text(_) => todo!(),
		RsxNode::Element(rsx_element) => todo!(),
	}
}
