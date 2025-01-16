use crate::prelude::*;
use std::marker::PhantomData;


pub struct RsxTreeWalker<'a, R, V> {
	pub visitor: &'a mut V,
	_r: PhantomData<R>,
}

impl<'a, R, V: RsxTreeVisitor<R>> RsxTreeWalker<'a, R, V> {
	pub fn new(visitor: &'a mut V) -> Self {
		Self {
			visitor,
			_r: PhantomData,
		}
	}



	pub fn walk_nodes_dfs(
		&mut self,
		nodes: &mut Vec<Node<R>>,
	) -> ParseResult<()> {
		self.walk_nodes_dfs_with_options(nodes, false)
	}
	pub fn walk_nodes_dfs_with_options(
		&mut self,
		nodes: &mut Vec<Node<R>>,
		visit_children: bool,
	) -> ParseResult<()> {
		if visit_children {
			self.visitor.visit_children(nodes)?;
		}
		for node in nodes.iter_mut() {
			self.visitor.visit_node(node)?;
			if let Some(children) = node.children_mut() {
				self.walk_nodes_dfs_with_options(children, true)?;
			}
			self.visitor.leave_node(node)?;
		}
		if visit_children {
			self.visitor.leave_children(nodes)?;
		}
		Ok(())
	}
}

#[allow(unused_variables)]
pub trait RsxTreeVisitor<R> {
	fn visit_node(&mut self, node: &mut Node<R>) -> ParseResult<()> { Ok(()) }
	fn leave_node(&mut self, node: &mut Node<R>) -> ParseResult<()> { Ok(()) }
	fn visit_children(
		&mut self,
		children: &mut Vec<Node<R>>,
	) -> ParseResult<()> {
		Ok(())
	}
	fn leave_children(
		&mut self,
		children: &mut Vec<Node<R>>,
	) -> ParseResult<()> {
		Ok(())
	}
}

/// Track the position of items in a tree,
/// Other visitors that use this vistor must ensure the following
/// are called **after** using the values, see [RsxTreeMetaVisitor] for an example.
#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub struct RsxTreePositionVisitor {
	pub current_pos: TreePosition,
	/// total accumulated index,
	/// tells how many nodes were visited
	pub node_index: usize,
}

impl<R> RsxTreeVisitor<R> for RsxTreePositionVisitor {
	fn visit_node(&mut self, _node: &mut Node<R>) -> ParseResult<()> {
		self.node_index += 1;
		Ok(())
	}
	fn leave_node(&mut self, _node: &mut Node<R>) -> ParseResult<()> {
		self.current_pos.next_sibling();
		self.node_index -= 1;
		Ok(())
	}
	fn visit_children(
		&mut self,
		_children: &mut Vec<Node<R>>,
	) -> ParseResult<()> {
		self.current_pos.next_child();
		Ok(())
	}
	fn leave_children(
		&mut self,
		_children: &mut Vec<Node<R>>,
	) -> ParseResult<()> {
		self.current_pos.prev_child();
		Ok(())
	}
}

#[derive(Debug, Default, PartialEq)]
pub struct RsxTreeMetaVisitor {
	pub position_visitor: RsxTreePositionVisitor,
	pub nodes: Vec<RsxTreeMeta>,
}

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub struct RsxTreeMeta {
	pub type_name: String,
	/// some descriptive info depending on the node variant
	pub info: String,
	pub position: TreePosition,
	pub node_index: usize,
}

impl<R> RsxTreeVisitor<R> for RsxTreeMetaVisitor {
	fn visit_node(&mut self, node: &mut Node<R>) -> ParseResult<()> {
		self.nodes.push(RsxTreeMeta {
			type_name: node.as_ref().to_string(),
			info: match node {
				Node::Doctype => "doctype".to_string(),
				Node::Comment(val) => val.clone(),
				Node::Element(element) => element.tag.clone(),
				Node::Text(text) => text.clone(),
				Node::TextBlock(_) => "text block".to_string(),
				Node::Component(_, _) => "component".to_string(),
			},
			position: self.position_visitor.current_pos.clone(),
			node_index: self.position_visitor.node_index,
		});
		self.position_visitor.visit_node(node)?;
		Ok(())
	}
	fn leave_node(&mut self, _node: &mut Node<R>) -> ParseResult<()> {
		self.position_visitor.leave_node(_node)
	}
	fn visit_children(
		&mut self,
		children: &mut Vec<Node<R>>,
	) -> ParseResult<()> {
		self.position_visitor.visit_children(children)
	}
	fn leave_children(
		&mut self,
		children: &mut Vec<Node<R>>,
	) -> ParseResult<()> {
		self.position_visitor.leave_children(children)
	}
}



#[cfg(test)]
mod test {
	use crate as sweet;
	use crate::prelude::*;
	use sweet_rsx_macros::rsx;
	use sweet_test::prelude::*;

	#[test]
	fn position_visitor() {
		let mut tree = rsx! {
			<div>
				<br/>
			</div>
			<br/>
		};
		let mut visitor = RsxTreePositionVisitor::default();
		RsxTreeWalker::new(&mut visitor)
			.walk_nodes_dfs(&mut tree.nodes)
			.unwrap();
		expect(visitor.node_index).to_be(0);
		expect(&visitor.current_pos.to_csv()).to_be("2");
	}

	#[test]
	fn works() {
		let mut tree = rsx! {
			<div>
				<br/>
				<p>hello</p>
				<p>world</p>
			</div>
			<br/>
		};
		let mut visitor = RsxTreeMetaVisitor::default();
		RsxTreeWalker::new(&mut visitor)
			.walk_nodes_dfs(&mut tree.nodes)
			.unwrap();
		// println!("{:#?}", visitor);

		expect(visitor.nodes.len()).to_be(7);
		expect(&visitor.nodes[0].position.to_csv()).to_be("0");
		expect(&visitor.nodes[1].position.to_csv()).to_be("0,0");
		expect(&visitor.nodes[2].position.to_csv()).to_be("0,1");
		expect(&visitor.nodes[3].position.to_csv()).to_be("0,1,0");
		expect(&visitor.nodes[4].position.to_csv()).to_be("0,2");
		expect(&visitor.nodes[5].position.to_csv()).to_be("0,2,0");
		expect(&visitor.nodes[6].position.to_csv()).to_be("1");
		// expect(&visitor.nodes[1].position)
		// 	.to_be(&TreePosition::from_vec(vec![0, 0]));

		// let expected = RsxTreeMetaVisitor {
		// 	position_visitor: RsxTreePositionVisitor {
		// 		current_pos: TreePosition::from_vec(vec![1, 0]),
		// 		node_index: 5,
		// 	},
		// 	nodes: vec![RsxTreeMeta {
		// 		type_name: "Element".to_string(),
		// 		position: TreePosition::from_vec(vec![0]),
		// 		node_index: 0,
		// 		children: vec![
		// 			RsxTreeMeta {
		// 				type_name: "Element".to_string(),
		// 				position: TreePosition::from_vec(vec![0, 1]),
		// 				node_index: 1,
		// 				children: vec![RsxTreeMeta {
		// 					type_name: "Text".to_string(),
		// 					position: TreePosition::from_vec(vec![0, 2]),
		// 					node_index: 2,
		// 					children: vec![],
		// 				}],
		// 			},
		// 			RsxTreeMeta {
		// 				type_name: "Element".to_string(),
		// 				position: TreePosition::from_vec(vec![2, 0]),
		// 				node_index: 3,
		// 				children: vec![RsxTreeMeta {
		// 					type_name: "Text".to_string(),
		// 					position: TreePosition::from_vec(vec![2, 1]),
		// 					node_index: 4,
		// 					children: vec![],
		// 				}],
		// 			},
		// 		],
		// 	}],
		// };

		// expect(&visitor).to_be(&expected);

		// let expected = RsxTreeMeta::new(

		// let mut visitor = TestVisitor::default();
	}
}
