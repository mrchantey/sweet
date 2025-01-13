use crate::prelude::*;
use web_sys::Event;

pub type HydratedEvent = Box<dyn FnMut(Event)>;


/// Collection of all events and blocks to be bound
/// The vecs are a flattened version of all componennts in the tree,
/// active or otherwise
pub struct Hydrated {
	pub events: Vec<HydratedEvent>,
	pub blocks: Vec<HydratedBlock>,
}

pub struct HydratedBlock {
	pub node_id: usize,
	/// The index of the part in the node
	/// ie for `hello {name}` the part index would be 1
	pub part_index: usize,
}


pub trait HydrateClient {
	fn hydrate(
		self,
		send: flume::Sender<(usize, String)>,
	) -> ParseResult<Hydrated>;
}
