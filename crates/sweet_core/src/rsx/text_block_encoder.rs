use super::ElementIndex;
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
/// An element may have multiple collapsed text blocks,
/// for instance:
/// ```html
/// <div> the quick brown {animal} <b> jumps </b> over the {color} dog </div>
/// ```
#[derive(Debug, PartialEq)]
pub struct TextBlockEncoder {
	pub parent_id: ElementIndex,
	/// the index of the child text node that collapsed
	pub collapsed_child_index: usize,
	/// a vec of 'next index to split at'
	pub split_positions: Vec<usize>,
}

impl TextBlockEncoder {
	pub fn new(parent_id: ElementIndex) -> Self {
		Self {
			parent_id,
			collapsed_child_index: 0,
			split_positions: Vec::new(),
		}
	}


	/// Store the indices
	pub fn encode(id: ElementIndex, el: &RsxElement) -> Vec<Self> {
		let mut encoded_children = Vec::new();
		let mut current = Self::new(id);
		// the index is the child index and the value is a vec of 'next index to split at'
		// let indices: Vec<Vec<usize>> = Vec::new();
		for node in CollapsedNode::from_element(el) {
			match node {
				CollapsedNode::StaticText(t) => {
					current.split_positions.push(t.len());
				}
				CollapsedNode::RustText(t) => {
					current.split_positions.push(t.len());
				}
				CollapsedNode::Break => {
					if !current.split_positions.is_empty() {
						encoded_children.push(current);
						current = Self::new(id);
					}
				}
			}
		}
		encoded_children
	}

	pub fn to_csv(&self) -> String {
		format!(
			"{},{},{}",
			self.parent_id,
			self.collapsed_child_index,
			self.split_positions
				.iter()
				.map(|i| i.to_string())
				.collect::<Vec<String>>()
				.join("-")
		)
	}

	pub fn to_csv_file(items: &Vec<Self>) -> String {
		items
			.iter()
			.map(Self::to_csv)
			.collect::<Vec<String>>()
			.join("\n")
	}

	pub fn from_csv_file(file: &str) -> ParseResult<Vec<Self>> {
		file.lines().map(Self::from_csv).collect()
	}

	pub fn from_csv(line: &str) -> ParseResult<Self> {
		let mut items = line.split(",");
		let parent_id = items
			.next()
			.ok_or_else(|| ParseError::Serde("missing parent id".into()))?
			.parse()?;


		let collapsed_child_index = items
			.next()
			.ok_or_else(|| {
				ParseError::Serde("missing collapsed child index".into())
			})?
			.parse()?;
		let split_positions = items
			.next()
			.ok_or_else(|| ParseError::Serde("missing split positions".into()))?
			.split("-")
			.map(|i| i.parse::<usize>())
			.collect::<Result<Vec<usize>, _>>()?;

		Ok(Self {
			parent_id,
			collapsed_child_index,
			split_positions,
		})
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
				out.push(CollapsedNode::RustText(RsxToHtml::render(initial)));
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

		let tree = rsx! {<div>"The "{desc}" and "{color}<b> fox </b> {action}" the "<Adjective> and fat </Adjective>dog</div>};
		let RsxNode::Element(el) = &tree.children()[0] else {
			panic!("expected element");
		};


		let collapsed = CollapsedNode::from_element(&el);

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
			CollapsedNode::StaticText("dog".into()),
		]);

		let encoded = TextBlockEncoder::encode(0, &el);
		let csv = TextBlockEncoder::to_csv_file(&encoded);
		expect(&csv).to_be("0,0,4-5-5-5\n0,0,10-5-4");

		let decoded = TextBlockEncoder::from_csv_file(&csv).unwrap();
		expect(decoded).to_be(encoded);
	}
}
