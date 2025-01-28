use crate::prelude::*;


pub type ElementIndex = usize;
pub type RustNodeIndex = usize;


/// Descriptor for current position in the tree.
/// This is used for both rendering and hydrating
/// for fine-grained reconciliation.
#[derive(Debug, Clone)]
pub struct RsxContext {
	/// the number of rsx rust blocks visited,
	/// this is useful for hot reloading because it will not change
	/// even if the html structure changes
	rust_node_index: RustNodeIndex,
	/// the number of html elements visited,
	/// elements with rust children will need to have this assigned as an
	/// attribute so that the hydrator can find them
	html_element_index: ElementIndex,
	/// the *uncollapsed* index of this block relative to its parent element
	child_index: usize,
}

impl Default for RsxContext {
	fn default() -> Self {
		Self {
			child_index: 0,
			rust_node_index: 0,
			html_element_index: 0,
		}
	}
}

impl RsxContext {
	pub fn visit(
		node: &RsxNode,
		mut func: impl FnMut(&Self, &RsxNode),
	) -> Self {
		let mut visitor = Self::default();
		visitor.visit_node_recursive(node, &mut func);
		visitor
	}
	pub fn visit_mut(
		node: &mut RsxNode,
		mut func: impl FnMut(&mut Self, &mut RsxNode),
	) -> Self {
		let mut visitor = Self::default();
		visitor.visit_node_mut_recursive(node, &mut func);
		visitor
	}

	fn visit_node_recursive(
		&mut self,
		node: &RsxNode,
		func: &mut impl FnMut(&Self, &RsxNode),
	) {
		func(self, node);
		self.after_visit_node(&node.into_discriminant());
		for node in node.children() {
			self.visit_node_recursive(node, func);
		}
	}
	fn visit_node_mut_recursive(
		&mut self,
		node: &mut RsxNode,
		func: &mut impl FnMut(&mut Self, &mut RsxNode),
	) {
		func(self, node);
		self.after_visit_node(&node.into_discriminant());
		for node in node.children_mut() {
			self.visit_node_mut_recursive(node, func);
		}
	}

	pub fn rust_node_index(&self) -> usize { self.rust_node_index }
	pub fn html_element_index(&self) -> usize { self.html_element_index }
	pub fn child_index(&self) -> usize { self.child_index }

	pub fn after_visit_node(&mut self, node: &RsxNodeDiscriminants) {
		match node {
			RsxNodeDiscriminants::Block => {
				self.rust_node_index += 1;
			}
			RsxNodeDiscriminants::Element => {
				self.child_index = 0;
				self.html_element_index += 1;
			}
			_ => {}
		}
	}
}


impl std::fmt::Display for RsxContext {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Rsx Node Count: {}", self.rust_node_index)
	}
}
