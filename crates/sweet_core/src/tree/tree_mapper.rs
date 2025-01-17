use super::Node;
use super::TreePosition;
use crate::error::ParseResult;

#[allow(unused_variables)]
pub trait TreeMapper<I: Node, O> {
	// fn tree_position(&mut self) -> &TreePosition;
	fn position(&mut self) -> &mut TreePosition;

	/// iterate over nodes, mapping them
	/// while updating the [TreePositionVisitor]
	fn map_nodes(&mut self, nodes: Vec<I>) -> ParseResult<Vec<O>> {
		self.position().visit_children();
		let nodes = nodes
			.into_iter()
			.map(|node| {
				self.position().visit_node();
				let out = self.map_node(node);
				self.position().leave_node();
				out
			})
			.collect();
		self.position().leave_children();
		nodes
	}

	fn map_node(&mut self, node: I) -> ParseResult<O>;
}
