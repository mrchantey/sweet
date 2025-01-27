use crate::prelude::*;

/// Descriptor for current position in the tree.
/// This is used for both rendering and hydrating
/// for fine-grained reconciliation.
#[derive(Debug, Clone)]
pub struct RsxContext {
	/// the number of rsx nodes visited
	rsx_node_count: usize,
	/// the rsx id of the parent element
	// pub html_pos: TreePosition,
	/// an id for the current element
	pub element_id: usize,
	/// The index of this child *at rsx time*. This will be incorrect
	/// post reload because other children may be added or removed
	pub element_child_index: usize,
	/// in rsx the html may move around during a reload but the indices of
	/// child blocks is constant so we use that to track the position
	pub element_child_block_index: usize,
}

impl Default for RsxContext {
	fn default() -> Self {
		Self {
			rsx_node_count: 0,
			element_id: 0,
			element_child_index: 0,
			element_child_block_index: 0,
		}
	}
}


impl RsxContext {
	/// The rsx id of the current node
	pub fn rsx_id(&self) -> usize { self.rsx_node_count - 1 }


	// we do not recurse into fragments or elements,
	// as this function will be called for each of them
	pub fn before_visit_next(&mut self, node: &RsxNodeDiscriminants) {
		match node {
			RsxNodeDiscriminants::Fragment => {}
			RsxNodeDiscriminants::Block => {}
			RsxNodeDiscriminants::Text
			| RsxNodeDiscriminants::Doctype
			| RsxNodeDiscriminants::Comment => self.next_rsx_id(),
			RsxNodeDiscriminants::Element => {
				self.next_rsx_id();
				self.element_id = self.rsx_id();
			}
		}
	}
	/// call this before visiting the children of an element
	pub fn before_element_children(&mut self) {
		self.element_child_index = 0;
		self.element_child_block_index = 0;
		// self.html_pos.push_child();
	}

	/// call this after visiting the children of an element
	pub fn after_element_children(&mut self) {}

	pub fn after_visit_next(&mut self, node: &RsxNodeDiscriminants) {
		self.element_child_index += 1;
		match node {
			RsxNodeDiscriminants::Fragment => {}
			RsxNodeDiscriminants::Block => {
				self.element_child_block_index += 1;
			}
			RsxNodeDiscriminants::Text => {}
			RsxNodeDiscriminants::Doctype => {}
			RsxNodeDiscriminants::Comment => {}
			RsxNodeDiscriminants::Element => {}
		}
	}

	fn next_rsx_id(&mut self) { self.rsx_node_count += 1; }

}

impl std::fmt::Display for RsxContext {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Rsx Node Count: {}", self.rsx_node_count)
	}
}
