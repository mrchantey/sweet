#[cfg(feature = "serde")]
pub use serde::Deserialize;
#[cfg(feature = "serde")]
pub use serde::Serialize;

/// Very simple storage of html
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HtmlPartial {
	pub elements: Vec<Element>,
}


/// Minimum required info for our use case of html.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Element {
	pub tag: String,
	pub attributes: Vec<Attribute>,
	pub children: Vec<Node>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Node {
	Element(Element),
	Text(String),
}


#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Attribute {
	Key { key: String },
	KeyValue { key: String, value: String },
	BlockValue { key: String },
	Block,
}
