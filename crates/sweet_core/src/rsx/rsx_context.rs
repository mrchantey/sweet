use crate::prelude::*;
use std::borrow::Borrow;
use std::collections::VecDeque;
use strum_macros::EnumDiscriminants;


pub type ElementIndex = usize;
pub type RustNodeIndex = usize;


/// This is how we match the rsx tree to the html tree,
///
/// 1. Count number of rsx nodes visited, and use this as an rsx_id
/// 2. Track their position relative to a parent element
///
/// The algorithm will only work for breadth-first traversal
/// because of how we track the child indices
#[derive(Debug, Clone, PartialEq)]
pub struct RsxContext {
	/// the number of rsx rust blocks visited,
	/// this is useful for hot reloading because it will not change
	/// even if the html structure changes
	pub(crate) num_rust_blocks: RustNodeIndex,
	/// *Note* this will not
	/// the number of html elements visited,
	/// elements with rust children will need to have this assigned as an
	/// attribute so that the hydrator can find them.
	pub(crate) num_elements: ElementIndex,
	/// the *uncollapsed* index of this block relative to its parent element
	pub(crate) child_index: usize,
}

impl Default for RsxContext {
	fn default() -> Self {
		Self {
			child_index: 0,
			num_rust_blocks: 0,
			num_elements: 0,
		}
	}
}

impl RsxContext {
	pub fn to_csv(&self) -> String {
		format!(
			"{},{},{}",
			self.num_rust_blocks, self.num_elements, self.child_index
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
			num_rust_blocks: rust_node_index,
			num_elements: html_element_index,
			child_index,
		})
	}
	/// Breadth-first traversal of the rsx tree,
	/// identical impl to visit_mut
	pub fn visit(
		node: &RsxNode,
		mut func: impl FnMut(&Self, &RsxNode),
	) -> Self {
		let mut visitor = Self::default();
		visitor.visit_impl(
			node,
			|cx, node| {
				func(cx, node);
				node
			},
			|queue, node| match node {
				RsxNode::Fragment(rsx_nodes) => {
					for node in rsx_nodes {
						queue.push_back(HtmlElementPosition::Other(node));
					}
				}
				RsxNode::Block { initial, .. } => {
					queue.push_back(HtmlElementPosition::Other(initial));
				}
				RsxNode::Element(RsxElement { children, .. }) => {
					let num_children = children.len();
					for (i, child) in children.into_iter().enumerate() {
						queue.push_back(HtmlElementPosition::new_child(
							num_children,
							i,
							child,
						));
					}
				}
				_ => {}
			},
		);
		visitor
	}

	/// Breadth-first traversal of the rsx tree
	/// identical impl to visit	/// identical impl to visit_mut

	pub fn visit_mut(
		node: &mut RsxNode,
		mut func: impl FnMut(&Self, &mut RsxNode),
	) -> Self {
		let mut visitor = Self::default();
		visitor.visit_impl(
			node,
			|cx, node| {
				func(cx, node);
				node
			},
			|queue, node| match node {
				RsxNode::Fragment(rsx_nodes) => {
					for node in rsx_nodes {
						queue.push_back(HtmlElementPosition::Other(node));
					}
				}
				RsxNode::Block { initial, .. } => {
					queue.push_back(HtmlElementPosition::Other(initial));
				}
				RsxNode::Element(RsxElement { children, .. }) => {
					let num_children = children.len();
					for (i, child) in children.into_iter().enumerate() {
						queue.push_back(HtmlElementPosition::new_child(
							num_children,
							i,
							child,
						));
					}
				}
				_ => {}
			},
		);
		visitor
	}
	/// Breadth-first traversal of the rsx tree
	fn visit_impl<'a, T: Borrow<RsxNode>>(
		&mut self,
		node: T,
		mut func: impl FnMut(&mut Self, T) -> T,
		mut map_children: impl FnMut(&mut VecDeque<HtmlElementPosition<T>>, T),
	) {
		let mut queue = VecDeque::new();
		queue.push_back(HtmlElementPosition::FirstChild(node));

		while let Some(pos_node) = queue.pop_front() {
			let pos_disc = pos_node.discriminant();
			let node = pos_node.into_inner();
			let node_disc = node.borrow().discriminant();
			self.before_visit_node(&node_disc, &pos_disc);
			let num_children = node.borrow().children().len();
			let node = func(self, node);
			self.after_visit_node(&node_disc, &pos_disc, num_children);
			map_children(&mut queue, node);
		}
	}


	pub fn rust_node_index(&self) -> usize { self.num_rust_blocks }
	pub fn html_element_index(&self) -> usize { self.num_elements }
	pub fn child_index(&self) -> usize { self.child_index }

	fn before_visit_node(
		&mut self,
		_node_disc: &RsxNodeDiscriminants,
		pos_disc: &HtmlElementPositionDiscriminants,
	) {
		match pos_disc {
			HtmlElementPositionDiscriminants::FirstChild
			| HtmlElementPositionDiscriminants::OnlyChild => {
				self.child_index = 0;
			}
			_ => {}
		}
	}
	fn after_visit_node(
		&mut self,
		node_disc: &RsxNodeDiscriminants,
		pos_disc: &HtmlElementPositionDiscriminants,
		num_children: usize,
	) {
		match node_disc {
			RsxNodeDiscriminants::Block => {
				self.num_rust_blocks += 1;
			}
			RsxNodeDiscriminants::Fragment => {}
			RsxNodeDiscriminants::Element => {
				self.child_index += 1;
				if num_children == 0 {
					self.num_elements += 1;
				}
			}
			RsxNodeDiscriminants::Text
			| RsxNodeDiscriminants::Doctype
			| RsxNodeDiscriminants::Comment => {
				self.child_index += 1;
			}
		}
		match pos_disc {
			// only increment element index after visiting the last child
			HtmlElementPositionDiscriminants::LastChild
			| HtmlElementPositionDiscriminants::OnlyChild => {
				self.num_elements += 1;
			}
			_ => {}
		}
	}
}

#[derive(EnumDiscriminants)]
enum HtmlElementPosition<T> {
	FirstChild(T),
	LastChild(T),
	OnlyChild(T),
	Other(T),
}

impl<T> HtmlElementPosition<T> {
	fn new_child(num_children: usize, i: usize, child: T) -> Self {
		if num_children == 1 {
			HtmlElementPosition::OnlyChild(child)
		} else if i == 0 {
			HtmlElementPosition::FirstChild(child)
		} else if i == num_children - 1 {
			HtmlElementPosition::LastChild(child)
		} else {
			HtmlElementPosition::Other(child)
		}
	}


	fn discriminant(&self) -> HtmlElementPositionDiscriminants { self.into() }
	pub fn into_inner(self) -> T {
		match self {
			Self::FirstChild(val)
			| Self::LastChild(val)
			| Self::OnlyChild(val)
			| Self::Other(val) => val,
		}
	}
}


#[cfg(test)]
mod test {
	use crate::prelude::*;
	use sweet_rsx_macros::rsx;
	use sweet_test::prelude::*;


	#[test]
	fn csv() {
		let a = RsxContext {
			num_rust_blocks: 1,
			num_elements: 2,
			child_index: 3,
		};
		let csv = a.to_csv();
		let b = RsxContext::from_csv(&csv).unwrap();
		expect(a).to_be(b);
	}

	struct Child;
	impl Component for Child {
		fn render(self) -> impl Rsx {
			rsx! {<div>{8}</div>}
		}
	}

	#[test]
	fn rust_blocks() {
		expect(
			RsxContext::visit(&rsx! {<div></div>}, |_, _| {}).num_rust_blocks,
		)
		.to_be(0);
		expect(
			RsxContext::visit(&rsx! {{7}{8}{9}<Child/>}, |_, _| {})
				.num_rust_blocks,
		)
		.to_be(4);
	}

	#[test]
	fn elements() {
		expect(RsxContext::visit(&rsx! {<div></div>}, |_, _| {}).num_elements)
			.to_be(1);
		expect(
			RsxContext::visit(&rsx! {<div>738</div>}, |_, _| {}).num_elements,
		)
		.to_be(1);
		expect(
			RsxContext::visit(&rsx! {<div><b>pow</b></div><Child/>}, |_, _| {})
				.num_elements,
		)
		.to_be(3);
	}
}
