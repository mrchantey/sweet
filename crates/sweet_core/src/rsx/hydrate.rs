/// The event block, ie on_click
#[cfg(target_arch = "wasm32")]
pub type HydratedEvent = Box<dyn FnMut(web_sys::Event)>;
#[cfg(not(target_arch = "wasm32"))]
pub type HydratedEvent = Box<dyn FnMut(())>;

/// Description of the position of a block in an rsx tree
#[derive(Debug, Clone)]
pub struct HydratedTextBlock {
	/// The assigned incremental id, not html id
	pub node_id: usize,
	/// The index of the part in the node
	/// ie for `hello {name}` the part index would be 1
	pub part_index: usize,
	pub initial_value: String,
}
