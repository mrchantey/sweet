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
}


const PLACEHOLDER: char = '§';

impl<R: RsxRust> RsxTree<R> {
	/// placeholder for rust parts

	pub fn extend(&mut self, other: Self) {
		let Self { nodes } = other;
		self.nodes.extend(nodes);
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
	pub fn build_string(&self) -> String {
		match self {
			RsxNode::Doctype => "<!DOCTYPE html>".to_string(),
			RsxNode::Comment(s) => format!("<!--{}-->", s),
			RsxNode::Element(e) => e.build_string(),
			RsxNode::Text(s) => s.clone(),
			RsxNode::Block(_) => PLACEHOLDER.to_string(),
		}
	}
}

// impl<R: RsxRust> Node for RsxNode<R> {

// 	fn children(&self) -> Option<&Vec<RsxNode<R>>> {
// 		match self {
// 			RsxNode::Element(e) => Some(&e.children),
// 			_ => None,
// 		}
// 	}
// 	fn children_mut(&mut self) -> Option<&mut Vec<RsxNode<R>>> {
// 		match self {
// 			RsxNode::Element(e) => Some(&mut e.children),
// 			_ => None,
// 		}
// 	}
// 	fn take_children(&mut self) -> Option<Vec<RsxNode<R>>> {
// 		match self {
// 			RsxNode::Element(e) => Some(std::mem::take(&mut e.children)),
// 			_ => None,
// 		}
// 	}
// }


/// Minimum required info for our use case of html.
/// Blocks are assumed to be `PartiaEq` because
/// they are defined as 'the next block in the vec' when reconciling.
// #[derive(Clone, PartialEq)]
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
		let self_closing = if self.self_closing { "/" } else { "" };

		out.push_str(&format!("<{}{}>", self.tag, self_closing));
		for attribute in &self.attributes {
			out.push(' ');
			out.push_str(&attribute.to_string_placeholder());
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
	pub fn to_string_placeholder(&self) -> String {
		match self {
			RsxAttribute::Key { key } => key.clone(),
			RsxAttribute::KeyValue { key, value } => {
				format!("{}=\"{}\"", key, value)
			}
			RsxAttribute::BlockValue { key, .. } => {
				format!("{}=\"{}\"", key, PLACEHOLDER)
			}
			RsxAttribute::Block(_) => PLACEHOLDER.to_string(),
		}
	}
}
