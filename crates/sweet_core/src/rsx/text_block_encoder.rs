use super::Node;
use super::RustParts;
use crate::error::ParseError;
use crate::error::ParseResult;

/// This module is for handling rsx text blocks in html text node.
///
/// ## The Problem
///
/// consider the following rsx:
/// ``` ignore
/// # use sweet_rsx_macros::rsx;
/// # use crate as sweet;
/// let desc = "quick";
/// let color = "brown";
/// let action = "jumps over";
/// let Adjective = rsx!{ lazy };
/// let Phrase = rsx!{ <div>The {desc} and {color} <b>fox</b> {action} the <Adjective/> dog</div> };
/// ```
/// This will flatten to the following html:
/// ```html
/// <div>The quick and brown <b>fox</b> jumps over the lazy dog</div>
/// ```
/// This encoder will encode the text block positions in the text node.
pub struct TextBlockEncoder;

impl TextBlockEncoder {
	/// Encoding into a 'dash comma dot' format
	/// Encoding for TextBlock positions, we need the following:
	/// - The child index of the text node
	/// - The string index of the block
	/// - The length of the TextBlock initial value
	/// child_index - first-block-index , first-block-length , second-block-index , second-block-length . child_index2 etc
	///
	/// ## Example
	/// ```html
	/// <div>the 10th <bold>value</bold> was 9</div>
	/// ```
	/// Output:
	/// 0-4,2.2-5,1
	pub fn encode(nodes: &Vec<Node<RustParts>>) -> String {
		let collapsed = CollapsedNode::from_nodes(nodes);
		Self::encode_text_block_positions(&collapsed)
	}

	fn encode_text_block_positions(nodes: &Vec<CollapsedNode>) -> String {
		let mut encoded = String::new();
		let mut child_index = 0;
		let mut text_index = 0;
		for node in nodes {
			match node {
				CollapsedNode::StaticText(t) => {
					text_index += t.len();
				}
				CollapsedNode::RustText(len) => {
					let len = len.len();
					encoded.push_str(&format!(
						"{}-{}-{},",
						child_index, text_index, len
					));
					text_index += len;
				}
				CollapsedNode::Break => {
					child_index += 1;
					text_index = 0;
				}
			}
		}
		if encoded.len() > 0 {
			encoded.pop();
		}
		encoded
	}

	pub fn decode(encoded: &str) -> ParseResult<Vec<TextBlockPosition>> {
		let mut out = Vec::new();
		let err = |_| {
			ParseError::Serde(format!(
				"Failed to decode text block positions from attribute: {}",
				encoded
			))
		};
		for block in encoded.split(",") {
			let mut parts = block.split("-");
			let child_index =
				parts.next().unwrap().parse::<usize>().map_err(err)?;
			let text_index =
				parts.next().unwrap().parse::<usize>().map_err(err)?;
			let len = parts.next().unwrap().parse::<usize>().map_err(err)?;
			out.push(TextBlockPosition {
				child_index,
				text_index,
				len,
			});
		}
		Ok(out)
	}
}



#[derive(Debug, Clone, PartialEq)]
enum CollapsedNode {
	/// static text, ie `rsx!{"foo"}`
	StaticText(String),
	/// text that can change, ie `rsx!{{val}}`
	RustText(String),
	/// doctype, comment, and element all break text node
	/// ie `rsx!{<div/>}`
	Break,
}
impl CollapsedNode {
	#[allow(unused)]
	pub(crate) fn as_str(&self) -> &str {
		match self {
			CollapsedNode::StaticText(val) => val,
			CollapsedNode::RustText(val) => val,
			CollapsedNode::Break => "|",
		}
	}
}

impl CollapsedNode {
	fn from_nodes(nodes: &Vec<Node<RustParts>>) -> Vec<CollapsedNode> {
		let mut out = Vec::new();
		for node in nodes {
			match node {
				Node::TextBlock(RustParts::TextBlock(val)) => {
					out.push(CollapsedNode::RustText(val.clone()))
				}
				Node::Text(val) => {
					out.push(CollapsedNode::StaticText(val.clone()))
				}
				Node::Doctype => out.push(CollapsedNode::Break),
				Node::Comment(_) => out.push(CollapsedNode::Break),
				Node::Element(_) => out.push(CollapsedNode::Break),
				Node::Component(RustParts::Component(children), vec) => {
					out.append(&mut Self::from_nodes(&children.nodes));
					out.append(&mut Self::from_nodes(vec))
				}
				_ => {
					// ignore invalid nodes
				}
			}
		}
		return out;
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextBlockPosition {
	pub child_index: usize,
	pub text_index: usize,
	pub len: usize,
}

#[cfg(test)]
mod test {
	#![allow(non_snake_case)]
	use crate as sweet;
	use crate::prelude::*;
	use sweet_rsx_macros::rsx;
	use sweet_test::prelude::*;
	use super::*;

	struct Adjective;
	impl Component for Adjective {
		fn render(self) -> impl Rsx {
			rsx! {"lazy"}
		}
	}

	#[test]
	fn roundtrip() {
		let desc = "quick";
		let color = "brown";
		let action = "jumps over";
		let tree = rsx! {"The "{desc}" and "{color}<b> fox </b> {action}" the "<Adjective> and fat </Adjective>dog };
		let collapsed = CollapsedNode::from_nodes(&tree.nodes);

		expect(&collapsed).to_be(&vec![
			CollapsedNode::StaticText("The ".into()),
			CollapsedNode::RustText("quick".into()),
			CollapsedNode::StaticText(" and ".into()),
			CollapsedNode::RustText("brown".into()),
			CollapsedNode::Break,
			CollapsedNode::RustText("jumps over".into()),
			CollapsedNode::StaticText(" the ".into()),
			CollapsedNode::StaticText("lazy".into()),
			CollapsedNode::StaticText(" and fat ".into()),
			CollapsedNode::StaticText("dog".into()),
		]);

		// println!(
		// 	"{}",
		// 	collapsed.iter().map(|n| n.as_str()).collect::<String>()
		// );
		let encoded = TextBlockEncoder::encode_text_block_positions(&collapsed);
		// println!("{}", encoded);
		expect(&encoded).to_be("0-4-5,0-14-5,1-0-10");

		let decoded = TextBlockEncoder::decode(&encoded).unwrap();


		expect(&decoded).to_be(&vec![
			TextBlockPosition {
				child_index: 0,
				text_index: 4,
				len: 5,
			},
			TextBlockPosition {
				child_index: 0,
				text_index: 14,
				len: 5,
			},
			TextBlockPosition {
				child_index: 1,
				text_index: 0,
				len: 10,
			},
		]);
	}
}
