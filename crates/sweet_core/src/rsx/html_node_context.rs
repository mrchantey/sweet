use crate::prelude::*;


#[derive(Debug, Clone)]
pub struct HtmlNodeContext {
	/// the number of rsx nodes visited
	rsx_node_count: usize,
	/// the rsx id of the parent element
	html_pos: TreePosition,

	/// if this is true and a text node is visited,
	/// the child index will not increment as these will collapse
	/// in html. The positioning will be encoded by [TextBlockEncoder]
	prev_text_node: bool,
}

impl Default for HtmlNodeContext {
	fn default() -> Self {
		Self {
			rsx_node_count: 0,
			html_pos: TreePosition::root(),
			prev_text_node: false,
		}
	}
}


impl HtmlNodeContext {

	/// The rsx id of the current node
	/// # Panics
	/// if no nodes have been visited
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
			RsxNodeDiscriminants::Element => self.visit_non_text_node(),
		}
	}
	/// call this before visiting the children of an element
	pub fn before_element_children(&mut self) {
		self.prev_text_node = false;
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
		self.prev_text_node = false;
		self.html_pos.next_sibling();
		self.next_rsx_id();
	}

	/// visit a text node that may be collapsed
	fn visit_text_node(&mut self) {
		if !self.prev_text_node {
			// this will create a new child
			self.prev_text_node = true;
			self.html_pos.next_sibling();
		}
		// otherwise this node will collapse into prev
		self.next_rsx_id();
	}
}

impl std::fmt::Display for HtmlNodeContext {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Rsx Node Count: {}", self.rsx_node_count)
	}
}
