use super::Node;
use super::TreeVisitor;
use crate::error::ParseResult;
use anyhow::anyhow;
use anyhow::Result;

pub trait PositionVisitor {
	fn position(&self) -> &TreePosition;
	fn position_mut(&mut self) -> &mut TreePosition;
}

impl PositionVisitor for TreePosition {
	fn position(&self) -> &TreePosition { self }
	fn position_mut(&mut self) -> &mut TreePosition { self }
}

impl<N: Node> TreeVisitor<N> for TreePosition {}

/// Track the position of items in a tree,
/// these methods should be 'outer' to any visitors,
/// ie call `visit_node` before visiting children, and `leave_node` after
/// and call `visit_children` before visiting children, and `leave_children` after
///
/// Represents the position of a node in the tree.
/// This always has at least one element.
///
/// Considering the following:
/// ```html
/// <html data-sweet-pos="0">
/// 	<head data-sweet-pos="0,0"></head>
/// 	<body data-sweet-pos="0,1">
/// 		<div data-sweet-pos="0,1,0"></div>
/// 		<div data-sweet-pos="0,1,1"></div>
/// 	</body>
/// </html>
/// ```
///
///
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct TreePosition {
	/// Vec of child indices, ie:
	/// [0,2,1] means: first child -> third child -> second child
	path: Vec<usize>,
	/// a number that increments when a node is visited,
	/// this will be '1' when the first child is visited,
	/// so we use current_index() - 1 to get the current index
	node_count: usize,
}

impl TreePosition {
	pub fn path(&self) -> &Vec<usize> { &self.path }
	/// The node count - 1
	/// # Panics
	/// If no nodes have been visited
	pub fn index(&self) -> usize { self.node_count - 1 }
	pub fn node_count(&self) -> usize { self.node_count }
}

impl TreePosition {
	pub fn visit_node(&mut self) { self.node_count += 1; }
	/// `path.last++, index--`
	/// # Panics
	/// if there are no positions
	pub fn leave_node(&mut self) {
		*self.path.last_mut().expect("tree is empty") += 1;
		// self.node_count -= 1;
	}
	// pub fn prev_sibling(&mut self) { *self.0.last_mut().unwrap() -= 1; }
	/// `path.push(0)`
	pub fn visit_children(&mut self) { self.path.push(0) }
	/// `path.pop()`
	/// # Panics
	/// if there are no positions
	pub fn leave_children(&mut self) { self.path.pop(); }
}
impl TreePosition {
	pub fn new() -> Self { Self::default() }
	/// # Panics
	/// if there are no positions, or if the last position is already 0
	pub fn push_child(&mut self) { self.path.push(0); }
	pub fn pop_child(&mut self) { self.path.pop(); }
	/// Convert to a comma separated value string, with the first index
	/// representing the **node count**, not index.
	/// ie "1,1,2"
	pub fn to_csv(&self) -> String {
		let mut values = vec![self.node_count.to_string()];
		values.extend(self.path.iter().map(|i| i.to_string()));
		values.join(",")
	}

	/// Tree position from comma separated values, ie "1,1,2"
	/// # Errors
	/// - if not empty and node count is zero
	/// - if any of the values are not parsable as usize
	pub fn from_csv(csv: &str) -> anyhow::Result<Self> {
		let values: Vec<usize> = csv
			.split(",")
			.map(|s| {
				s.parse().map_err(|e| {
					anyhow!("failed to parse csv for TreePosition: {s}\n{}", e)
				})
			})
			.collect::<Result<Vec<_>>>()?;

		if values.is_empty() {
			return Ok(Self {
				path: vec![],
				node_count: 0,
			});
		}
		if values[0] == 0 {
			return Err(anyhow!("node count cannot be zero"));
		}

		Ok(Self {
			node_count: values[0],
			path: values[1..].to_vec(),
		})
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

impl PositionVisitor for TreePositionCollector {
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
