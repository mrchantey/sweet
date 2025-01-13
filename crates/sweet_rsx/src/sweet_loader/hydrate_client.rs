use crate::prelude::*;

pub type HydratedEvent = Box<dyn Fn()>;


/// Collection of all events and blocks to be bound
/// The vecs are a flattened version of all componennts in the tree,
/// active or otherwise
pub struct Hydrated {
	pub events: Vec<Box<HydratedEvent>>,
}


pub trait HydrateClient {
	fn hydrate() -> ParseResult<Hydrated>;
}
