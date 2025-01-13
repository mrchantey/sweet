use crate::prelude::*;
use web_sys::Event;

pub type HydratedEvent = Box<dyn FnMut(Event)>;


/// Collection of all events and blocks to be bound
/// The vecs are a flattened version of all componennts in the tree,
/// active or otherwise
pub struct Hydrated {
	pub events: Vec<HydratedEvent>,
}


pub trait HydrateClient {
	fn hydrate() -> ParseResult<Hydrated>;
}
