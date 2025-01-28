use crate::prelude::*;
use std::collections::VecDeque;


pub type ElementIndex = usize;
pub type RustNodeIndex = usize;


/// Descriptor for current position in the tree.
/// This is used for both rendering and hydrating
/// for fine-grained reconciliation.
#[derive(Debug, Clone, PartialEq)]
pub struct RsxContext {
	/// the number of rsx rust blocks visited,
	/// this is useful for hot reloading because it will not change
	/// even if the html structure changes
	pub rust_node_index: RustNodeIndex,
	/// the number of html elements visited,
	/// elements with rust children will need to have this assigned as an
	/// attribute so that the hydrator can find them
	pub html_element_index: ElementIndex,
	/// the *uncollapsed* index of this block relative to its parent element
	pub child_index: usize,
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
	pub fn to_csv(&self) -> String {
		format!(
			"{},{},{}",
			self.rust_node_index, self.html_element_index, self.child_index
		)
	}
	pub fn from_csv(csv: &str) -> ParseResult<Self> {
		let mut parts = csv.split(',');
		let rust_node_index = parts
			.next()
			.ok_or_else(|| ParseError::serde("missing rust node index"))?
			.parse()?;
		let html_element_index = parts
			.next()
			.ok_or_else(|| ParseError::serde("missing rust node index"))?
			.parse()?;
		let child_index = parts
			.next()
			.ok_or_else(|| ParseError::serde("missing rust node index"))?
			.parse()?;
		Ok(Self {
			rust_node_index,
			html_element_index,
			child_index,
		})
	}


	pub fn visit(
		node: &RsxNode,
		mut func: impl FnMut(&Self, &RsxNode),
	) -> Self {
		let mut visitor = Self::default();
		let mut queue = VecDeque::new();
		queue.push_back(node);

		while let Some(node) = queue.pop_front() {
			func(&visitor, node);
			visitor.after_visit_node(&node);
			for child in node.children() {
				queue.push_back(child);
			}
		}


		visitor
	}
	pub fn visit_mut(
		node: &mut RsxNode,
		mut func: impl FnMut(&mut Self, &mut RsxNode),
	) -> Self {
		let mut visitor = Self::default();
		let mut queue = VecDeque::new();
		queue.push_back(node);

		while let Some(node) = queue.pop_front() {
			func(&mut visitor, node);
			visitor.after_visit_node(&node);
			for child in node.children_mut() {
				queue.push_back(child);
			}
		}


		visitor
	}

	pub fn rust_node_index(&self) -> usize { self.rust_node_index }
	pub fn html_element_index(&self) -> usize { self.html_element_index }
	pub fn child_index(&self) -> usize { self.child_index }

	pub fn after_visit_node(&mut self, node: &RsxNode) {
		self.track_child_idx(node);
		match node {
			RsxNode::Block { .. } => {
				self.rust_node_index += 1;
			}
			RsxNode::Element(_) => {
				self.html_element_index += 1;
			}
			_ => {}
		}
	}

	fn track_child_idx(&mut self, node: &RsxNode) {
		match node {
			RsxNode::Block { initial, .. } => {
				self.track_child_idx(initial);
			}
			RsxNode::Element(_) => {
				self.child_index = 0;
			}
			RsxNode::Fragment(chidren) => {
				for child in chidren {
					self.track_child_idx(child);
				}
			}
			RsxNode::Doctype => {
				self.child_index += 1;
			}
			RsxNode::Comment(_) => {
				self.child_index += 1;
			}
			RsxNode::Text(_) => {
				self.child_index += 1;
			}
		}
	}
}


impl std::fmt::Display for RsxContext {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Rsx Node Count: {}", self.rust_node_index)
	}
}


#[cfg(test)]
mod test {
	use crate::prelude::*;

	#[test]
	fn csv() {
		let a = RsxContext {
			rust_node_index: 1,
			html_element_index: 2,
			child_index: 3,
		};
		let csv = a.to_csv();
		let b = RsxContext::from_csv(&csv).unwrap();
		expect(a).to_be(b);
	}
}
