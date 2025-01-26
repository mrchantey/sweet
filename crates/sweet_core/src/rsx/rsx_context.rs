use crate::prelude::*;

/// Descriptor for current position in the tree.
/// This is used for both rendering and hydrating
/// for fine-grained reconciliation.
#[derive(Debug, Clone)]
pub struct RsxContext {
	/// the number of rsx nodes visited
	rsx_node_count: usize,
	/// the rsx id of the parent element
	pub html_pos: TreePosition,
	/// an id for the current element
	pub element_id: usize,
	/// the actual index of the html node child.
	/// This is post collapse so you may need to
	/// use [TextBlockEncoder] to split the node
	pub element_child_index: usize,
	/// text nodes may sit alongside one another
	/// in the [HtmlNode] tree so use this value to index
	/// before collapse
	pub element_child_index_uncollapsed: usize,

	/// if this is true and a text node is visited,
	/// the child index will not increment as these will collapse
	/// in html. The positioning will be encoded by [TextBlockEncoder]
	prev_text_node: bool,
}

impl Default for RsxContext {
	fn default() -> Self {
		Self {
			rsx_node_count: 0,
			element_id: 0,
			element_child_index: 0,
			element_child_index_uncollapsed: 0,
			html_pos: TreePosition::root(),
			prev_text_node: false,
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
			RsxNodeDiscriminants::Text => self.visit_text_node(),
			RsxNodeDiscriminants::Doctype => self.visit_non_text_node(),
			RsxNodeDiscriminants::Comment => self.visit_non_text_node(),
			RsxNodeDiscriminants::Element => {
				self.visit_non_text_node();
				self.element_id = self.rsx_id();
			}
		}
	}
	/// call this before visiting the children of an element
	pub fn before_element_children(&mut self) {
		self.prev_text_node = false;
		self.element_child_index = 0;
		self.element_child_index_uncollapsed = 0;
		self.html_pos.push_child();
	}

	/// call this after visiting the children of an element
	pub fn after_element_children(&mut self) {
		self.prev_text_node = false;
		self.html_pos.pop_child();
	}

	pub fn after_visit_next(&mut self, node: &RsxNodeDiscriminants) {
		match node {
			RsxNodeDiscriminants::Fragment => {}
			RsxNodeDiscriminants::Block => {}
			RsxNodeDiscriminants::Text => {}
			RsxNodeDiscriminants::Doctype => {}
			RsxNodeDiscriminants::Comment => {}
			RsxNodeDiscriminants::Element => {}
		}
	}

	fn next_rsx_id(&mut self) { self.rsx_node_count += 1; }

	/// visit a real html node
	fn visit_non_text_node(&mut self) {
		self.element_child_index += 1;
		self.element_child_index_uncollapsed += 1;
		self.prev_text_node = false;
		self.html_pos.next_sibling();
		self.next_rsx_id();
	}
	
	/// visit a text node that may be collapsed
	fn visit_text_node(&mut self) {
		self.element_child_index_uncollapsed += 1;
		if !self.prev_text_node {
			// this will create a new child
			self.prev_text_node = true;
			self.element_child_index += 1;
			self.html_pos.next_sibling();
		}
		// otherwise this node will collapse into prev
		self.next_rsx_id();
	}
}

impl std::fmt::Display for RsxContext {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Rsx Node Count: {}", self.rsx_node_count)
	}
}
