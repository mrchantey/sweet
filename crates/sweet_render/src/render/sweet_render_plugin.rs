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
	pub placeholder: String,
}
impl Default for SweetRenderPlugin {
	fn default() -> Self {
		Self {
			current_node: 0,
			placeholder: RsxParts::default_placeholder(),
		}
	}
}

impl RenderPlugin for SweetRenderPlugin {
	fn render(mut self, rsx: impl Rsx) -> ParseResult<String> {
		let html = self.render_recursive(rsx)?;
		Ok(html)
	}
}

impl SweetRenderPlugin {
	/// The render function will parse the parent node
	///
	///
	/// Breadth-first traversal of children,
	/// incrementing id
	fn render_recursive(&mut self, rsx: impl Rsx) -> ParseResult<String> {
		let RsxParts { rust, html, css } = rsx.into_parts();

		let _ = css;

		let html_in = html.load()?;

		// for <div>$<div> this is len_2
		let mut static_html = html_in.split(&self.placeholder);
		let Some(first_static) = static_html.next() else {
			// empty
			return Ok(html_in);
		};
		let mut html_out = first_static.to_string();

		// let placeholder_offsets = html_in
		// 	.match_indices(&self.placeholder)
		// 	.map(|(i, _)| i)
		// 	.collect::<Vec<_>>();

		// assumes same number of symbols as rust parts
		for (next_rust, next_static) in rust.into_iter().zip(static_html) {
			let replacement = match next_rust {
				RsxRust::DynNodeId => {
					self.current_node += 1;
					let out = format!("data-sid=\"{}\"", self.current_node);
					out
				}
				RsxRust::InnerText(_s) => {
					// resolve
					todo!()
				}
				RsxRust::AttributeKey(_s) => {
					todo!()
				}
				RsxRust::AttributeValue(_s) => {
					todo!()
				}
				RsxRust::Event(_e) => {
					format!("_sweet.event({},event)", self.current_node)
				}
				RsxRust::ChildComponent(c) => {
					self.render_recursive(c)?;
					todo!("render child, join child html")
				}
			};
			html_out.push_str(&replacement);
			html_out.push_str(next_static);
		}
		Ok(html_out)
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
