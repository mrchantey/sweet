mod html_node;
use crate::rsx::RsxContext;
use crate::rsx::RsxNode;
pub use html_node::*;
use std::cell::RefCell;



thread_local! {
	static HYDRATED: RefCell<Box<dyn Hydrated>> = RefCell::new(Box::new(HydratedHtml::new(vec![], HtmlConstants::default())));
}

#[derive(Debug, thiserror::Error)]
pub enum HydrationError {
	#[error("Invalid context: {0}")]
	InvalidContext(String),
	#[error("Invalid element: {0}")]
	InvalidElement(String),
}

impl HydrationError {
	pub fn invalid_context(msg: &str) -> Self {
		HydrationError::InvalidContext(msg.to_string())
	}
	pub fn invalid_element(msg: &str) -> Self {
		HydrationError::InvalidElement(msg.to_string())
	}
}

pub struct Hydrate;

impl Hydrate {
	pub fn with<R>(mut func: impl FnMut(&mut dyn Hydrated) -> R) -> R {
		HYDRATED.with(|hydrated| {
			let mut hydrated = hydrated.borrow_mut();
			func(hydrated.as_mut())
		})
	}


	pub fn set(item: impl 'static + Sized + Hydrated) {
		HYDRATED.with(|hydrated| {
			*hydrated.borrow_mut() = Box::new(item);
		});
	}
}


pub trait Hydrated {
	fn update_rsx_node(
		&mut self,
		node: RsxNode,
		cx: &RsxContext,
	) -> Result<(), HydrationError>;
	/// just used for testing atm
	fn render(&self) -> String;
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
