#[cfg(feature = "serde")]
pub use serde::Deserialize;
#[cfg(feature = "serde")]
pub use serde::Serialize;

/// Very simple storage of html, without any
/// parsing or validation, for use with rstml output.
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HtmlPartial {
	pub nodes: Vec<Node>,
}


impl HtmlPartial {
	/// placeholder for rust blocks
	pub const PLACEHOLDER: char = '§';
	pub fn new() -> Self { Self::default() }

	pub fn extend(&mut self, other: Self) {
		let Self { nodes } = other;
		self.nodes.extend(nodes);
	}

	pub fn to_string_placeholder(&self) -> String {
		let mut out = String::new();
		for node in &self.nodes {
			out.push_str(&node.to_string_placeholder());
		}
		out
	}
}


/// Minimum required info for our use case of html.
/// Blocks are assumed to be `PartiaEq` because
/// they are defined as 'the next block in the vec' when reconciling.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Element {
	/// ie `div, span, input`
	pub tag: String,
	/// ie `class="my-class"`
	pub attributes: Vec<Attribute>,
	/// ie `<div>childtext<childel/>{childblock}</div>`
	pub children: Vec<Node>,
	/// ie `<input/>`
	pub self_closing: bool,
}

impl Element {
	pub fn new(tag: String, self_closing: bool) -> Self {
		Self {
			tag,
			self_closing,
			attributes: Vec::new(),
			children: Vec::new(),
		}
	}


	/// Whether any children or attributes are blocks,
	/// used to determine whether the node requires an id
	pub fn contains_blocks(&self) -> bool {
		self.children.iter().any(|c| matches!(c, Node::TextBlock))
			|| self.attributes.iter().any(|a| {
				matches!(a, Attribute::Block | Attribute::BlockValue { .. })
			})
	}


	pub fn to_string_placeholder(&self) -> String {
		let mut out = String::new();
		let self_closing = if self.self_closing { "/" } else { "" };

		out.push_str(&format!("<{}{}>", self.tag, self_closing));
		for attribute in &self.attributes {
			out.push(' ');
			out.push_str(&attribute.to_string_placeholder());
		}
		for child in &self.children {
			out.push_str(&child.to_string_placeholder());
		}
		if !self.self_closing {
			out.push_str(&format!("</{}>", self.tag));
		}
		out
	}
}

/// a 'collapsed' rstml node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Node {
	Doctype,
	Comment(String),
	Element(Element),
	/// may have been Text or RawText
	Text(String),
	/// a rust block, contents is reconciled by renderer
	TextBlock,
	/// an rust value that implements [Rsx] contents is reconciled by renderer
	/// The children here are the 'children' of the component
	Component(Vec<Node>),
}

impl Node {
	pub fn to_string_placeholder(&self) -> String {

		match self {
			Node::Doctype => "<!DOCTYPE html>".to_string(),
			Node::Comment(s) => format!("<!--{}-->", s),
			Node::Element(e) => e.to_string_placeholder(),
			Node::Text(s) => s.clone(),
			Node::TextBlock => HtmlPartial::PLACEHOLDER.to_string(),
			Node::Component(c) => {
				c.iter().map(|c| c.to_string_placeholder()).collect()
			}
		}
	}
}


#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Attribute {
	Key { key: String },
	KeyValue { key: String, value: String },
	BlockValue { key: String },
	Block,
}

impl Attribute {
	pub fn to_string_placeholder(&self) -> String {
		match self {
			Attribute::Key { key } => key.clone(),
			Attribute::KeyValue { key, value } => {
				format!("{}=\"{}\"", key, value)
			}
			Attribute::BlockValue { key } => {
				format!("{}=\"{}\"", key, HtmlPartial::PLACEHOLDER)
			}
			Attribute::Block => HtmlPartial::PLACEHOLDER.to_string(),
		}
	}
}
