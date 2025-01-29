use crate::prelude::*;
mod html_node_hydrator;
mod rsx_context_map;
pub use html_node_hydrator::*;
pub use rsx_context_map::*;
use std::cell::RefCell;


#[cfg(target_arch = "wasm32")]
mod dom_hydrator;
#[cfg(target_arch = "wasm32")]
pub use dom_hydrator::*;

thread_local! {
	static CURRENT_HYDRATOR: RefCell<Box<dyn Hydrator>> = RefCell::new(Box::new(HtmlNodeHydrator::new((), HtmlConstants::default())));
}
pub struct CurrentHydrator;

impl CurrentHydrator {
	pub fn with<R>(mut func: impl FnMut(&mut dyn Hydrator) -> R) -> R {
		CURRENT_HYDRATOR.with(|current| {
			let mut current = current.borrow_mut();
			func(current.as_mut())
		})
	}


	pub fn set(item: impl 'static + Sized + Hydrator) {
		CURRENT_HYDRATOR.with(|current| {
			*current.borrow_mut() = Box::new(item);
		});
	}
}

pub trait Hydrator {
	fn update_rsx_node(
		&mut self,
		node: RsxNode,
		cx: &RsxContext,
	) -> ParseResult<()>;
	/// just used for testing atm
	fn render(&self) -> String;
}
