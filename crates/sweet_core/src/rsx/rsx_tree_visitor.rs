use crate::prelude::*;

#[allow(unused_variables)]
pub trait RsxTreeVisitor<R> {
	fn walk_nodes_dfs(&mut self, nodes: &Vec<Node<R>>) -> ParseResult<()> {
		self.visit_children(nodes)?;
		for node in nodes.iter() {
			self.visit_node(node)?;
			if let Some(children) = node.children() {
				self.walk_nodes_dfs(children)?;
			}
			self.leave_node(node)?;
		}
		self.leave_children(nodes)?;
		Ok(())
	}
	fn visit_node(&mut self, node: &Node<R>) -> ParseResult<()> { Ok(()) }
	fn leave_node(&mut self, node: &Node<R>) -> ParseResult<()> { Ok(()) }
	fn visit_children(&mut self, children: &Vec<Node<R>>) -> ParseResult<()> {
		Ok(())
	}
	fn leave_children(&mut self, children: &Vec<Node<R>>) -> ParseResult<()> {
		Ok(())
	}
}
#[allow(unused_variables)]
pub trait RsxTreeVisitorMut<R> {
	fn walk_nodes_dfs(&mut self, nodes: &mut Vec<Node<R>>) -> ParseResult<()> {
		self.visit_children(nodes)?;
		for node in nodes.iter_mut() {
			self.visit_node(node)?;
			if let Some(children) = node.children_mut() {
				self.walk_nodes_dfs(children)?;
			}
			self.leave_node(node)?;
		}
		self.leave_children(nodes)?;
		Ok(())
	}
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
/// this vistors methods should be 'outer' to any implementors
///
///
///
#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub struct RsxTreePositionVisitor {
	pub current_pos: TreePosition,
	/// number of rsx nodes visited, this is more than number of html nodes
	/// because of components and sibling text and textblock nodes
	pub node_count: usize,
}

impl RsxTreePositionVisitor {
	/// # Panics
	/// If no nodes have been visited
	pub fn current_node_id(&self) -> usize { self.node_count - 1 }
}

impl<R> RsxTreeVisitor<R> for RsxTreePositionVisitor {
	fn visit_node(&mut self, _node: &Node<R>) -> ParseResult<()> {
		self.node_count += 1;
		Ok(())
	}
	fn leave_node(&mut self, _node: &Node<R>) -> ParseResult<()> {
		self.current_pos.next_sibling();
		self.node_count -= 1;
		Ok(())
	}
	fn visit_children(&mut self, _children: &Vec<Node<R>>) -> ParseResult<()> {
		self.current_pos.push_child();
		Ok(())
	}
	fn leave_children(&mut self, _children: &Vec<Node<R>>) -> ParseResult<()> {
		self.current_pos.pop_child();
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
	/// called before visiting children
	fn visit_node(&mut self, node: &Node<R>) -> ParseResult<()> {
		self.position_visitor.visit_node(node)?;
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
			node_index: self.position_visitor.node_count,
		});
		Ok(())
	}
	/// called after visiting children
	fn leave_node(&mut self, _node: &Node<R>) -> ParseResult<()> {
		self.position_visitor.leave_node(_node)
	}
	fn visit_children(&mut self, children: &Vec<Node<R>>) -> ParseResult<()> {
		self.position_visitor.visit_children(children)
	}
	fn leave_children(&mut self, children: &Vec<Node<R>>) -> ParseResult<()> {
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
		visitor.walk_nodes_dfs(&mut tree.nodes).unwrap();
		expect(visitor.node_count).to_be(0);
		expect(&visitor.current_pos.to_csv()).to_be("");
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
		visitor.walk_nodes_dfs(&mut tree.nodes).unwrap();
		// println!("{:#?}", visitor);
		expect(visitor.nodes.len()).to_be(7);
		expect(&visitor.nodes[0].position.to_csv()).to_be("0");
		expect(&visitor.nodes[1].position.to_csv()).to_be("0,0");
		expect(&visitor.nodes[2].position.to_csv()).to_be("0,1");
		expect(&visitor.nodes[3].position.to_csv()).to_be("0,1,0");
		expect(&visitor.nodes[4].position.to_csv()).to_be("0,2");
		expect(&visitor.nodes[5].position.to_csv()).to_be("0,2,0");
		expect(&visitor.nodes[6].position.to_csv()).to_be("1");
	}
}
