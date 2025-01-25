#[cfg(feature = "serde")]
pub use serde::Deserialize;
#[cfg(feature = "serde")]
pub use serde::Serialize;
/// This struct represents one of the core concepts of sweet rsx!
///
// #[derive(Debug, Clone, PartialEq)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RsxNode {
	/// A transparent node that simply contains children
	Fragment(Vec<RsxNode>),
	/// a rust block that returns text
	TextBlock {
		initial: String,
		register_effect: Box<dyn FnOnce()>,
	},
	Doctype,
	Comment(String),
	/// may have been Text or RawText
	Text(String),
	Element(RsxElement),
}

impl Default for RsxNode {
	fn default() -> Self { Self::Fragment(Vec::new()) }
}


impl RsxNode {
	/// A method used by macros to insert nodes into a slot
	/// # Panics
	/// If the slot is not found
	pub fn with_slots(mut self, name: &str, nodes: Vec<RsxNode>) -> Self {
		match self.try_insert_slots(name, nodes) {
			Some(_) => {
				panic!("slot not found: {name}");
			}
			None => return self,
		}
	}

	// try to insert nodes into a slot, returning any nodes that were not inserted
	// If the slot is not a direct child, recursively search children
	pub fn try_insert_slots(
		&mut self,
		name: &str,
		mut nodes: Vec<Self>,
	) -> Option<Vec<Self>> {
		match self {
			RsxNode::Fragment(fragment) => {
				for node in fragment.iter_mut() {
					match node.try_insert_slots(name, nodes) {
						Some(returned_nodes) => nodes = returned_nodes,
						None => return None,
					}
				}
				Some(nodes)
			}
			RsxNode::Element(element) => {
				if element.tag == "slot" {
					let slot_name = element
						.attributes
						.iter()
						.find_map(|a| match a {
							RsxAttribute::KeyValue { key, value } => {
								if key == "name" {
									Some(value.as_str())
								} else {
									None
								}
							}
							// even block values are not allowed because we need slot names at macro time
							_ => None,
						})
						// unnamed slots are called 'default'
						.unwrap_or("default");
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

	pub fn register_effects(self) {
		match self {
			RsxNode::TextBlock {
				register_effect, ..
			} => register_effect(),
			RsxNode::Element(e) => {
				for a in e.attributes {
					match a {
						RsxAttribute::Block {
							register_effect, ..
						} => register_effect(),
						RsxAttribute::BlockValue {
							register_effect, ..
						} => register_effect(),
						_ => {}
					}
				}
				for c in e.children {
					c.register_effects();
				}
			}
			RsxNode::Fragment(nodes) => {
				for n in nodes {
					n.register_effects();
				}
			}
			_ => {}
		}
	}

	pub fn build_string(&self) -> String {
		match self {
			RsxNode::Doctype => "<!DOCTYPE html>".to_string(),
			RsxNode::Comment(s) => format!("<!--{}-->", s),
			RsxNode::Element(e) => e.build_string(),
			RsxNode::Text(s) => s.clone(),
			RsxNode::TextBlock { initial, .. } => initial.clone(),
			RsxNode::Fragment(nodes) => {
				nodes.iter().map(|n| n.build_string()).collect::<String>()
			}
		}
	}
}

/// Minimum required info for our use case of html.
/// Blocks are assumed to be `PartiaEq` because
/// they are defined as 'the next block in the vec' when reconciling.
// #[derive(Debug, Clone)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RsxElement {
	/// ie `div, span, input`
	pub tag: String,
	/// ie `class="my-class"`
	pub attributes: Vec<RsxAttribute>,
	/// ie `<div>childtext<childel/>{childblock}</div>`
	pub children: Vec<RsxNode>,
	/// ie `<input/>`
	pub self_closing: bool,
}


impl RsxElement {
	pub fn new(tag: String, self_closing: bool) -> Self {
		Self {
			tag,
			self_closing,
			attributes: Vec::new(),
			children: Vec::new(),
		}
	}

	pub fn contains_text_blocks(&self) -> bool {
		self.children
			.iter()
			.any(|c| matches!(c, RsxNode::TextBlock { .. }))
	}

	/// Whether any children or attributes are blocks,
	/// used to determine whether the node requires an id
	pub fn contains_rust(&self) -> bool {
		self.contains_text_blocks()
			|| self.attributes.iter().any(|a| {
				matches!(
					a,
					RsxAttribute::Block { .. }
						| RsxAttribute::BlockValue { .. }
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
pub enum RsxAttribute {
	Key {
		key: String,
	},
	KeyValue {
		key: String,
		value: String,
	},
	BlockValue {
		key: String,
		initial: String,
		register_effect: Box<dyn FnOnce()>,
	},
	Block {
		initial: Vec<RsxAttribute>,
		register_effect: Box<dyn FnOnce()>,
	},
}

impl RsxAttribute {
	pub fn build_string(&self) -> String {
		match self {
			RsxAttribute::Key { key } => key.clone(),
			RsxAttribute::KeyValue { key, value } => {
				if key == "slot" {
					String::default()
				} else {
					format!("{}=\"{}\"", key, value)
				}
			}
			RsxAttribute::BlockValue { key, initial, .. } => {
				format!("{}=\"{}\"", key, initial)
			}
			RsxAttribute::Block { initial, .. } => {
				initial.iter().map(|a| a.build_string()).collect::<String>()
			}
		}
	}
}
