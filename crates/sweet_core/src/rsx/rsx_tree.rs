use crate::prelude::*;
#[cfg(feature = "serde")]
pub use serde::Deserialize;
#[cfg(feature = "serde")]
pub use serde::Serialize;
/// This struct represents one of the core concepts of sweet rsx!
///
/// It is a type that represents a tree of html, but with the
/// rusty parts represented as <R>.
///
/// This allows us to convert between hydrated and serialized html trees.
///
/// There are currently three types being used (in order of process step):
/// - [RsxTree<TokenStream>]: for macros and preprocessing
/// - [RsxTree<RustParts>]: for rendering
/// - [RsxTree<()>]: for serialization
///
/// Here we have actual rust code, stored as boxed closures and strings,
///
/// A data structure for rsx trees, with the option
/// of ommiting rust parts for serialization.
/// may or may not contain rust parts, depending on the value of R.
/// R will be either unit for serialization or [RustParts](super::RustParts)
// #[derive(Debug, Clone, PartialEq)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RsxTree<R: RsxRust> {
	pub nodes: Vec<RsxNode<R>>,
}
impl<R: RsxRust> Default for RsxTree<R> {
	fn default() -> Self { Self { nodes: Vec::new() } }
}

impl<R: RsxRust> RsxTree<R> {
	pub fn new(nodes: Vec<RsxNode<R>>) -> Self { Self { nodes } }

	pub fn build_string(&self) -> String {
		let mut out = String::new();
		for node in &self.nodes {
			out.push_str(&node.build_string());
		}
		out
	}
}

impl<R: RsxRust> RsxTree<R> {
	/// placeholder for rust parts

	pub fn extend(&mut self, other: Self) {
		let Self { nodes } = other;
		self.nodes.extend(nodes);
	}
	/// A method used by macros to insert nodes into a slot
	/// # Panics
	/// If the slot is not found
	pub fn with_slots(
		mut self,
		name: &str,
		mut nodes: Vec<RsxNode<R>>,
	) -> Self {
		for node in self.nodes.iter_mut() {
			match node.try_insert_slots(name, nodes) {
				Some(returned_nodes) => nodes = returned_nodes,
				None => return self,
			}
		}
		panic!("slot not found: {name}");
	}

	// pub fn build_string(&self) -> String {
	// 	let mut out = String::new();
	// 	for node in &self.nodes {
	// 		out.push_str(&node.info());
	// 	}
	// 	out
	// }
}

/// a 'collapsed' rstml node
// #[derive(Debug, Clone, PartialEq, AsRefStr)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RsxNode<R: RsxRust> {
	Doctype,
	Comment(String),
	Element(RsxElement<R>),
	/// may have been Text or RawText
	Text(String),
	/// a rust block, contents is reconciled by renderer
	Block(R::Block),
}

impl<R: RsxRust> RsxNode<R> {
	// try to insert nodes into a slot, returning any nodes that were not inserted
	// If the slot is not a direct child, recursively search children
	pub fn try_insert_slots(
		&mut self,
		name: &str,
		mut nodes: Vec<Self>,
	) -> Option<Vec<Self>> {
		match self {
			RsxNode::Element(element) => {
				if element.tag == "slot" {
					let slot_name = element
						.attributes
						.iter()
						.find_map(|a| match a {
							RsxAttribute::KeyValue { key, value } => {
								if key == "name" {
									Some(value.clone())
								} else {
									None
								}
							}
							RsxAttribute::BlockValue { key, value } => {
								if key == "name" {
									let value =
										R::attribute_block_value_to_string(
											value,
										);
									Some(value)
								} else {
									None
								}
							}
							_ => None,
						})
						// unnamed slots are called 'default'
						.unwrap_or("default".to_string());
					if slot_name == name {
						element.children.extend(nodes);
						return None;
					}
				}
				// if we didnt find the slot, recursively search children
				for child in &mut element.children {
					match child.try_insert_slots(name, nodes) {
						Some(returned_nodes) => nodes = returned_nodes,
						None => return None,
					}
				}
				Some(nodes)
			}
			_ => Some(nodes),
		}
	}

	pub fn build_string(&self) -> String {
		match self {
			RsxNode::Doctype => "<!DOCTYPE html>".to_string(),
			RsxNode::Comment(s) => format!("<!--{}-->", s),
			RsxNode::Element(e) => e.build_string(),
			RsxNode::Text(s) => s.clone(),
			RsxNode::Block(block) => R::block_to_string(block),
		}
	}
}

/// Minimum required info for our use case of html.
/// Blocks are assumed to be `PartiaEq` because
/// they are defined as 'the next block in the vec' when reconciling.
// #[derive(Debug, Clone)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RsxElement<R: RsxRust> {
	/// ie `div, span, input`
	pub tag: String,
	/// ie `class="my-class"`
	pub attributes: Vec<RsxAttribute<R>>,
	/// ie `<div>childtext<childel/>{childblock}</div>`
	pub children: Vec<RsxNode<R>>,
	/// ie `<input/>`
	pub self_closing: bool,
}


impl<R: RsxRust> RsxElement<R> {
	pub fn new(tag: String, self_closing: bool) -> Self {
		Self {
			tag,
			self_closing,
			attributes: Vec::new(),
			children: Vec::new(),
		}
	}

	pub fn contains_text_blocks(&self) -> bool {
		self.children.iter().any(|c| matches!(c, RsxNode::Block(_)))
	}

	/// Whether any children or attributes are blocks,
	/// used to determine whether the node requires an id
	pub fn contains_rust(&self) -> bool {
		self.contains_text_blocks()
			|| self.attributes.iter().any(|a| {
				matches!(
					a,
					RsxAttribute::Block(_) | RsxAttribute::BlockValue { .. }
				)
			})
	}


	pub fn build_string(&self) -> String {
		let mut out = String::new();
		// slots are a kind of fragment, just return children
		if self.tag == "slot" {
			for child in &self.children {
				out.push_str(&child.build_string());
			}
			return out;
		}


		out.push_str(&format!("<{}", self.tag));
		for attribute in &self.attributes {
			out.push(' ');
			out.push_str(&attribute.build_string());
		}
		if self.self_closing {
			out.push_str("/>");
			return out;
		} else {
			out.push('>');
		}
		for child in &self.children {
			out.push_str(&child.build_string());
		}
		if !self.self_closing {
			out.push_str(&format!("</{}>", self.tag));
		}
		out
	}
}

// #[derive(Debug, Clone, PartialEq)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RsxAttribute<R: RsxRust> {
	Key {
		key: String,
	},
	KeyValue {
		key: String,
		value: String,
	},
	BlockValue {
		key: String,
		value: R::AttributeBlockValue,
	},
	Block(R::AttributeBlock),
}

impl<R: RsxRust> RsxAttribute<R> {
	pub fn build_string(&self) -> String {
		match self {
			RsxAttribute::Key { key } => key.clone(),
			RsxAttribute::KeyValue { key, value } => {
				format!("{}=\"{}\"", key, value)
			}
			RsxAttribute::BlockValue { key, value } => {
				format!(
					"{}=\"{}\"",
					key,
					R::attribute_block_value_to_string(value)
				)
			}
			RsxAttribute::Block(block) => R::attribute_block_to_string(block),
		}
	}
}
