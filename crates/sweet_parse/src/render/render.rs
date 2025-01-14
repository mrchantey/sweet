


///	Encoding for text and blocks.
/// The 'nodes' provided by rstml are not real, ie text, rawtext and block nodes will
/// be mashed into a single text node. We need to track the position of the block nodes
/// in the original string so it can be split up again by the renderer.
/// The format is like so
///
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





pub fn render(){



}






// /// Interim encoding for block positions.
// /// each block is a double: child index, start index
// /// # Note:
// /// Raw text nodes are trimmed
// fn encode_text_block_positions<C>(children: &Vec<Node<C>>) -> String {
// 	let mut encoded = String::new();
// 	let mut child_index = 0;
// 	let mut text_index = 0;
// 	for child in children {
// 		match child {
// 			Node::RawText(t) => {
// 				text_index += t.to_string_best().len();
// 			}
// 			Node::Text(t) => {
// 				text_index += t.value_string().len();
// 			}
// 			Node::Block(_) => {
// 				encoded.push_str(&format!("{},{},", child_index, text_index));
// 			}
// 			_ => {
// 				if text_node_end(child) {
// 					child_index += 1;
// 					text_index = 0;
// 					continue;
// 				}
// 			}
// 		}
// 	}
// 	if encoded.len() > 0 {
// 		encoded.pop();
// 	}
// 	encoded
// }
