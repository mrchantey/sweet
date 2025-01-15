use super::RsxVisitor;
use sweet_core::error::ParseResult;
use sweet_core::rsx::Attribute;
use sweet_core::rsx::Element;
use sweet_core::rsx::Node;






#[derive(Default)]
pub struct SweetRsxVisitor {
	pub current_rust: usize,
	pub num_dyn_elements: usize,
}

impl RsxVisitor for SweetRsxVisitor {
	fn visit_rust(
		&mut self,
		_rust: &mut sweet_core::prelude::RsxRust,
	) -> ParseResult<()> {
		self.current_rust += 1;
		Ok(())
	}

	fn visit_element(&mut self, element: &mut Element) -> ParseResult<()> {
		if !element.contains_blocks() {
			return Ok(());
		}
		element.attributes.push(Attribute::KeyValue {
			key: "data-sweet-id".to_string(),
			value: self.num_dyn_elements.to_string(),
		});
		self.num_dyn_elements += 1;
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


	fn visit_event_attribute(
		&mut self,
		_key: &mut String,
		value: &mut String,
	) -> ParseResult<()> {
		*value = format!(
			"_sweet.event({},event)",
			self.num_dyn_elements - 1
		);
		Ok(())
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
fn encode_text_block_positions(children: &Vec<Node>) -> String {
	let mut encoded = String::new();
	let mut child_index = 0;
	let mut text_index = 0;
	for child in children {
		match child {
			Node::Text(t) => {
				text_index += t.len();
			}
			Node::TextBlock => {
				encoded.push_str(&format!("{},{},", child_index, text_index));
			}
			Node::Component(_) => {
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
