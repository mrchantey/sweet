use super::RsxElement;
use super::RsxNode;
use crate::error::ParseError;
use crate::error::ParseResult;
use crate::html::RsxToHtml;

/// This module is for handling rsx text blocks in html text node.
///
/// The tricky part of resumability encoding the *minimal* amount of information
/// in html, the first version of quik relied heavily on using `<-- COMMENTS -->` to
/// break up text nodes but this bloats html size very quickly.
/// Instead this encoder uses the bare minimum information more closely resembling
/// the quik 2.0 proposal https://www.builder.io/blog/qwik-2-coming-soon
///
///
///
///
///
///
pub struct TextBlockEncoder;

impl TextBlockEncoder {
	/// Store the
	pub fn encode(el: &RsxElement) -> String {
		let collapsed = CollapsedNode::from_element(el);
		Self::encode_text_block_positions(&collapsed)
	}

	fn encode_text_block_positions(nodes: &Vec<CollapsedNode>) -> String {
		let mut encoded = String::new();
		let mut child_index = 0;
		let mut text_index = 0;

		// the index is the child index and the value is a vec of 'next index to split at'
		// let indices: Vec<Vec<usize>> = Vec::new();
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
		let err = |_| {
			ParseError::Serde(format!(
				"Failed to decode text block positions from attribute: {}",
				encoded
			))
		};

		let mut out = Vec::new();

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
	fn from_element(el: &RsxElement) -> Vec<CollapsedNode> {
		el.children.iter().flat_map(Self::from_node).collect()
	}
	fn from_node(node: &RsxNode) -> Vec<CollapsedNode> {
		let mut out = Vec::new();
		match node {
			RsxNode::Fragment(nodes) => {
				out.extend(nodes.into_iter().flat_map(Self::from_node));
			}
			RsxNode::Block { initial, .. } => {
				out.push(CollapsedNode::RustText(RsxToHtml::render(
					initial,
				)));
			}
			RsxNode::Text(val) => {
				out.push(CollapsedNode::StaticText(val.clone()))
			}
			RsxNode::Doctype => out.push(CollapsedNode::Break),
			RsxNode::Comment(_) => out.push(CollapsedNode::Break),
			RsxNode::Element(_) => out.push(CollapsedNode::Break),
		}
		return out;
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextBlockPosition {
	/// the actual node index of the html parent element
	pub child_index: usize,
	/// the starting index of the text block
	pub text_index: usize,
	/// the length of the text block
	pub len: usize,
}

impl TextBlockPosition {
	/// returns a vec where the indices are the child indexes,
	/// and the values are a text index and length of each block
	/// Block positions at 0 are ignored
	pub fn into_split_positions(
		positions: Vec<TextBlockPosition>,
	) -> Vec<Vec<usize>> {
		let mut out = Vec::new();
		for pos in positions {
			let child = {
				if let Some(child) = out.get_mut(pos.child_index) {
					child
				} else {
					out.resize(pos.child_index + 1, Vec::new());
					out.last_mut().unwrap()
				}
			};
			if pos.text_index > 0 {
				child.push(pos.text_index);
			}
			child.push(pos.text_index + pos.len);
		}
		out
	}
}


#[cfg(test)]
mod test {
	use super::*;
	use crate::prelude::*;
	use sweet_rsx_macros::rsx;

	struct Adjective;
	impl Component for Adjective {
		fn render(self) -> impl Rsx {
			rsx! {"lazy"<slot/>}
		}
	}

	#[test]
	fn roundtrip() {
		let desc = "quick";
		let color = "brown";
		let action = "jumps over";

		// let tree = rsx! {"The "{desc}" and "{color}<b> fox </b> {action}" the "<Adjective> and fat </Adjective>dog };
		let tree = rsx! {"The "{desc}" and "{color}<b> fox </b> {action}" the "<Adjective> and fat </Adjective>dog };
		let collapsed = CollapsedNode::from_node(&tree);

		expect(&collapsed).to_be(&vec![
			CollapsedNode::StaticText("The ".into()),
			CollapsedNode::RustText("quick".into()),
			CollapsedNode::StaticText(" and ".into()),
			CollapsedNode::RustText("brown".into()),
			CollapsedNode::Break,
			CollapsedNode::RustText("jumps over".into()),
			CollapsedNode::StaticText(" the ".into()),
			CollapsedNode::StaticText("lazy".into()),
			CollapsedNode::Break,
			// CollapsedNode::StaticText(" and fat ".into()),
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

		let indices = TextBlockPosition::into_split_positions(decoded);
		expect(&indices).to_be(&vec![vec![4, 9, 14, 19], vec![10]]);
	}
}
