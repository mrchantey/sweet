use crate::prelude::*;
use sweet_core::prelude::*;



/// The `SweetRenderPlugin` is the second part to the `SweetRsxPlugin`.
/// 
/// It makes a few transformations:
/// - collect the html templates for all children
/// 
/// 
/// # Parsing algorithm
/// 
/// 1. search the input html for the placeholder
/// 2. when one is found, append all html up to that point to the output
/// 3. check for double placeholder, if so its beginning of a child block 
/// 4. 
/// 
pub struct SweetRenderPlugin {
	pub current_node: usize,
	pub html: String,
	pub placeholder: String,
}
impl Default for SweetRenderPlugin {
	fn default() -> Self {
		Self {
			current_node: 0,
			html: String::new(),
			placeholder: RsxParts::default_placeholder(),
		}
	}
}

impl RenderPlugin for SweetRenderPlugin {
	fn render(mut self, rsx: impl Rsx) -> ParseResult<String> {
		self.render_recursive(rsx)?;
		Ok(self.html)
	}
}

impl SweetRenderPlugin {
	/// The render function will parse the parent node
	///
	///
	/// Breadth-first traversal of children,
	/// incrementing id
	fn render_recursive(&mut self, rsx: impl Rsx) -> ParseResult<()> {
		let RsxParts {
			rust,
			mut html,
			css,
		} = rsx.into_parts();

		let mut html = html.load()?;
		let placeholder_offsets = html
			.match_indices(&self.placeholder)
			.map(|(i, _)| i)
			.enumerate();

		let mut rust = rust.into_iter();



		let mut incr_offset = 0;

		for (count, index) in placeholder_offsets {
			let adjusted_index = index + incr_offset;

			let next = rust.next().ok_or_else(|| {
				ParseError::Hydration(format!(
					"found a placeholder but no matching rust block"
				))
			})?;
			let replacement = match next {
				RsxRust::DynNodeId => {
					let out = format!("data-sid=\"{}\"", self.current_node);
					self.current_node += 1;
					out
				}
				RsxRust::InnerText(s) => {
					todo!()
				}
				RsxRust::AttributeKey(s) => {
					todo!()
				}
				RsxRust::AttributeValue(s) => {
					todo!()
				}
				RsxRust::Event(e) => {
					format!("_sweet.event({},event)", self.current_node)
				}
				RsxRust::ChildComponent(c) => {
					todo!("render child, join child html")
					// self.render(c);
				}
			};
			incr_offset += replacement.len() - 1;
		}
		Ok(())
	}
}


#[cfg(test)]
mod test {
	// use super::SweetRenderPlugin;
	// use crate::render::RenderPlugin;
	use sweet::prelude::*;

	#[test]
	fn works() {
		let onclick = |_| {};
		let world = "mars";
		let rsx = rsx! {
			<div onclick>
				// <p>hello {world}</p>
			</div>
		};

		let rendered = SweetRenderPlugin::default().render(rsx).unwrap();
		println!("html: '{}'", rendered);

		// expect(true).to_be_false();
	}
}





// ///	Encoding for text and blocks.
// /// The 'nodes' provided by rstml are not real, ie text, rawtext and block nodes will
// /// be mashed into a single text node. We need to track the position of the block nodes
// /// in the original string so it can be split up again by the renderer.
// /// The format is like so
// ///
// /// child_index - first-block-index , first-block-length , second-block-index , second-block-length . child_index2 etc
// ///
// /// ## Example
// /// ```html
// /// <div>the 10th <bold>value</bold> was 9</div>
// /// ```
// /// Output:
// /// 0-4,2.2-5,1
// ///
// ///


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
