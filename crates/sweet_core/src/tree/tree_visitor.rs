use crate::prelude::*;

#[allow(unused_variables)]
pub trait TreeVisitor<N: Node>: PositionVisitor {
	fn walk_nodes_dfs(&mut self, nodes: &Vec<N>) -> ParseResult<()> {
		self.position_mut().visit_children();
		self.visit_children(nodes)?;
		for node in nodes.iter() {
			self.position_mut().visit_node();
			self.visit_node(node)?;
			if let Some(children) = node.children() {
				self.walk_nodes_dfs(children)?;
			}
			self.leave_node(node)?;
			self.position_mut().leave_node();
		}
		self.leave_children(nodes)?;
		self.position_mut().leave_children();
		Ok(())
	}
	fn visit_node(&mut self, node: &N) -> ParseResult<()> { Ok(()) }
	fn leave_node(&mut self, node: &N) -> ParseResult<()> { Ok(()) }
	fn visit_children(&mut self, children: &Vec<N>) -> ParseResult<()> {
		Ok(())
	}
	fn leave_children(&mut self, children: &Vec<N>) -> ParseResult<()> {
		Ok(())
	}
}
#[allow(unused_variables)]
pub trait TreeVisitorMut<N: Node>: PositionVisitor {
	fn walk_nodes_dfs(&mut self, nodes: &mut Vec<N>) -> ParseResult<()> {
		self.position_mut().visit_children();
		self.visit_children(nodes)?;
		for node in nodes.iter_mut() {
			self.position_mut().visit_node();
			self.visit_node(node)?;
			if let Some(children) = node.children_mut() {
				self.walk_nodes_dfs(children)?;
			}
			self.leave_node(node)?;
			self.position_mut().leave_node();
		}
		self.leave_children(nodes)?;
		self.position_mut().leave_children();
		Ok(())
	}
	fn visit_node(&mut self, node: &mut N) -> ParseResult<()> { Ok(()) }
	fn leave_node(&mut self, node: &mut N) -> ParseResult<()> { Ok(()) }
	fn visit_children(&mut self, children: &mut Vec<N>) -> ParseResult<()> {
		Ok(())
	}
	fn leave_children(&mut self, children: &mut Vec<N>) -> ParseResult<()> {
		Ok(())
	}
}
#[allow(unused_variables)]
pub trait TreeVisitorOwned<N: Node>: PositionVisitor {
	fn walk_nodes_dfs(&mut self, nodes: Vec<N>) -> ParseResult<()> {
		self.position_mut().visit_children();
		let nodes = self.visit_children(nodes)?;
		for node in nodes.into_iter() {
			self.position_mut().visit_node();
			if let Some(children) = self.visit_node(node)? {
				self.walk_nodes_dfs(children)?;
			}
			self.leave_node()?;
			self.position_mut().leave_node();
		}
		self.leave_children()?;
		self.position_mut().leave_children();
		Ok(())
	}

	/// take a node, optionally returning its children
	fn visit_node(&mut self, mut node: N) -> ParseResult<Option<Vec<N>>> {
		Ok(node.take_children())
	}
	fn leave_node(&mut self) -> ParseResult<()> { Ok(()) }
	fn visit_children(&mut self, children: Vec<N>) -> ParseResult<Vec<N>> {
		Ok(children)
	}
	fn leave_children(&mut self) -> ParseResult<()> { Ok(()) }
}


