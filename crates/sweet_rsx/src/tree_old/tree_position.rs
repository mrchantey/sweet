use super::Node;
use super::TreeVisitor;
use crate::error::ParseResult;
use anyhow::anyhow;
use anyhow::Result;

pub trait VisitWithPosition {
	fn position(&self) -> &TreePosition;
	fn position_mut(&mut self) -> &mut TreePosition;
}

impl VisitWithPosition for TreePosition {
	fn position(&self) -> &TreePosition { self }
	fn position_mut(&mut self) -> &mut TreePosition { self }
}

impl<T: VisitWithPosition, N: Node> TreeVisitor<N> for T {
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
}



#[derive(Debug, Default, PartialEq)]
pub struct TreePositionCollector {
	pub position: TreePosition,
	pub nodes: Vec<TreeMeta>,
}

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub struct TreeMeta {
	pub type_name: String,
	/// some descriptive info depending on the node variant
	pub info: String,
	pub position: TreePosition,
	pub node_index: usize,
}

impl VisitWithPosition for TreePositionCollector {
	fn position(&self) -> &TreePosition { &self.position }
	fn position_mut(&mut self) -> &mut TreePosition { &mut self.position }
}

impl<N: Node> TreeVisitor<N> for TreePositionCollector {
	/// called before visiting children
	fn visit_node(&mut self, node: &N) -> ParseResult<()> {
		self.nodes.push(TreeMeta {
			type_name: node.variant().to_string(),
			info: node.info(),
			position: self.position.clone(),
			node_index: self.position.index(),
		});
		Ok(())
	}
}

#[cfg(test)]
mod test {
	use crate::prelude::*;


	#[test]
	fn position_visitor() {
		let mut tree = rsx! {
			<div>
				<br/>
			</div>
			<br/>
		};
		let mut position = TreePosition::default();
		position.walk_nodes_dfs(&mut tree.nodes).unwrap();
		expect(position.node_count()).to_be(3);
		expect(&position.to_csv()).to_be("3");
	}

	#[test]
	fn works() {
		let pos = TreePosition::default();
		expect(&pos.to_csv()).to_be("0");
	}

	#[test]
	fn collector() {
		let mut tree = rsx! {
			<div>
				<br/>
				<p>hello</p>
				<p>world</p>
			</div>
			<br/>
		};
		let mut visitor = TreePositionCollector::default();
		visitor.walk_nodes_dfs(&mut tree.nodes).unwrap();
		// println!("{:#?}", visitor);
		expect(visitor.nodes.len()).to_be(7);
		expect(&visitor.nodes[0].position.to_csv()).to_be("1,0");
		expect(&visitor.nodes[1].position.to_csv()).to_be("2,0,0");
		expect(&visitor.nodes[2].position.to_csv()).to_be("3,0,1");
		expect(&visitor.nodes[3].position.to_csv()).to_be("4,0,1,0");
		expect(&visitor.nodes[4].position.to_csv()).to_be("5,0,2");
		expect(&visitor.nodes[5].position.to_csv()).to_be("6,0,2,0");
		expect(&visitor.nodes[6].position.to_csv()).to_be("7,1");
	}
}
