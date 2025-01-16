use crate::prelude::*;
use sweet_core::prelude::*;





#[derive(Default)]
pub struct SweetRsxVisitor {
	pub current_pos: TreePosition,
	pub num_nodes: usize,
}

impl RsxVisitor for SweetRsxVisitor {
	fn visit_node(&mut self, _node: &mut Node<RustParts>) -> ParseResult<()> {
		self.current_pos.next_sibling();
		self.num_nodes += 1;
		Ok(())
	}

	fn visit_element(
		&mut self,
		element: &mut Element<RustParts>,
	) -> ParseResult<()> {
		let current_pos = self.current_pos.to_csv();
		self.current_pos.next_child();

		if !element.contains_blocks() {
			return Ok(());
		}
		element.attributes.push(Attribute::KeyValue {
			key: "data-sweet-pos".to_string(),
			value: current_pos,
		});

		// we encode text block positions here because we need to see all
		// children to calculate the positions
		let encoded_block_positions =
			encode_text_block_positions(&element.children);

		if !encoded_block_positions.is_empty() {
			element.attributes.push(Attribute::KeyValue {
				key: "data-sweet-blocks".to_string(),
				value: encoded_block_positions,
			});
		}

		// if let Some(encoded) = encode_text_block_positions(children) {
		// 	out.push(' ');
		// 	out.push_str(&encoded);
		// }
		Ok(())
	}


	fn visit_event_attribute(&mut self, _key: &str) -> ParseResult<String> {
		Ok(format!("_sweet.event({},event)", self.num_nodes - 1))
	}
}

///	Encoding for TextBlock positions, we need the following:
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
///
///
fn encode_text_block_positions(children: &Vec<Node<RustParts>>) -> String {
	let mut encoded = String::new();
	let mut child_index = 0;
	let mut text_index = 0;
	for child in children {
		match child {
			Node::Text(t) => {
				text_index += t.len();
			}
			Node::TextBlock(_) => {
				encoded.push_str(&format!("{},{},", child_index, text_index));
			}
			Node::Component(_, _) => {
				todo!("what if component returns text")
			}
			_ => {
				child_index += 1;
				text_index = 0;
				continue;
			}
		}
	}
	if encoded.len() > 0 {
		encoded.pop();
	}
	encoded
}
