use crate::prelude::*;
use flume::Sender;


/// The event block, ie on_click
#[cfg(target_arch = "wasm32")]
pub type HydratedEvent = Box<dyn FnMut(web_sys::Event)>;
#[cfg(not(target_arch = "wasm32"))]
pub type HydratedEvent = Box<dyn FnMut(())>;


pub enum HydratedRsxRust {
	Event(HydratedEvent),
	Block(HydratedTextBlock),
}

/// Collection of all events and blocks to be bound
/// The vecs are a flattened version of all componennts in the tree,
/// active or otherwise
pub struct Hydrated {
	pub rust: Vec<HydratedRsxRust>,
}

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


/// Trait for hydrating a client.
/// Usually this is not implemented directly, instead see [`sweet::rsx!`]
pub trait HydrateClient {
	fn hydrate(self, send: Sender<(usize, String)>) -> ParseResult<Hydrated>;
}

impl std::fmt::Debug for Hydrated {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Hydrated")
			.field("rust.len", &self.rust.len())
			// .field("blocks", &self.blocks)
			.finish()
	}
}
