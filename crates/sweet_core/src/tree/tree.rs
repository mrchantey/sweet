/// The base representation of a tree
/// Implementing these functions allow for
/// a tree to be traversed by visitors and mappers
pub trait Tree {
	type Node: Node;
}

pub trait Node: Sized {
	/// Optionally returns the variant of the node
	/// for debugging and diffing
	fn variant(&self) -> &'static str { "unimplemented" }
	/// Optionally returns the variant of the node
	/// for debugging and diffing
	fn info(&self) -> String { "unimplemented".into() }
	fn children(&self) -> Option<&Vec<Self>>;
	fn children_mut(&mut self) -> Option<&mut Vec<Self>>;
	fn take_children(&mut self) -> Option<Vec<Self>>;
}
