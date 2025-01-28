use crate::prelude::*;
use std::collections::HashMap;


/// This map is updated every hot reload, the position
/// of a rust block in the tree can change
#[derive(Debug, Default, Clone)]
pub struct RsxContextMap {
	pub rust_blocks: Vec<RsxContext>,
	pub collapsed_elements: HashMap<usize, Vec<usize>>,
}



impl RsxContextMap {
	pub fn from_node(node: &RsxNode) -> Self {
		let mut map = Self::default();

		let visitor = RsxContext::visit(node, |cx, node| match node {
			RsxNode::Block { .. } => {
				assert_eq!(cx.rust_node_index(), map.rust_blocks.len());
				map.rust_blocks.push(cx.clone());
			}
			RsxNode::Element(el)=>{
				if el.contains_blocks(){


				}
			}
			_ => {}
		});
		assert_eq!(visitor.rust_node_index(), map.rust_blocks.len());

		map
	}
}
