use super::Node;
use crate::error::ParseResult;

#[allow(unused_variables)]
pub trait RsxTreeMapper<I, O> {
	fn map_nodes(&mut self, nodes: Vec<Node<I>>) -> ParseResult<Vec<Node<O>>> {
		nodes.into_iter().map(|node| self.map_node(node)).collect()
	}

	fn map_node(&mut self, node: Node<I>) -> ParseResult<Node<O>>;
}
