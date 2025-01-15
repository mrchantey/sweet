use crate::prelude::*;
use sweet_core::prelude::*;



/// The `SweetRenderPlugin` is the second part to the `RsxParser`.
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
}
impl Default for SweetRenderPlugin {
	fn default() -> Self { Self { current_node: 0 } }
}

impl RenderPlugin for SweetRenderPlugin {
	fn render(mut self, rsx: impl Rsx) -> ParseResult<String> {
		let html = self.visit_rsx(rsx)?;
		Ok(html)
	}
}

impl SweetRenderPlugin {
	/// The render function will parse the parent node
	///
	///
	/// Breadth-first traversal of children,
	/// incrementing id
	fn visit_rsx(&mut self, rsx: impl Rsx) -> ParseResult<String> {
		let RsxParts { mut rust, html } = rsx.into_parts();
		let html = html.load()?;
		self.visit_nodes(&mut rust, html.nodes)
	}

	fn visit_nodes(
		&mut self,
		rust: &mut Vec<RsxRust>,
		nodes: Vec<Node>,
	) -> ParseResult<String> {
		let mut out = String::new();
		for node in nodes {
			out.push_str(&self.visit_node(rust, node)?);
		}
		Ok(out)
	}

	fn visit_node(
		&mut self,
		rust: &mut Vec<RsxRust>,
		node: Node,
	) -> ParseResult<String> {
		match node {
			Node::Doctype => Ok("<!DOCTYPE html>".to_string()),
			Node::Comment(val) => Ok(format!("<!-- {} -->", val)),
			Node::Text(val) => Ok(val),
			Node::Element(Element {
				tag,
				attributes,
				children,
				self_closing,
			}) => {
				let mut str = self.visit_element_open(
					rust,
					&tag,
					attributes,
					self_closing,
				)?;
				str.push_str(&self.visit_nodes(rust, children)?);
				str.push_str(&self.visit_element_close(&tag, self_closing)?);
				Ok(str)
			}
			Node::TextBlock => {
				if let Some(RsxRust::InnerText(val)) = rust.pop() {
					Ok(val)
				} else {
					Err(ParseError::hydration("expected text block"))
				}
			}
			Node::Component(element) => self.visit_component(rust, element),
		}
	}

	fn visit_component(
		&mut self,
		rust: &mut Vec<RsxRust>,
		Element {
			tag,
			attributes,
			children,
			self_closing,
		}: Element,
	) -> ParseResult<String> {
		if let Some(RsxRust::Component(component)) = rust.pop() {
			let mut str =
				self.visit_element_open(rust, &tag, attributes, self_closing)?;
			str.push_str(&self.visit_rsx(component)?);
			// TODO This is incorrect, children should be passed to the component
			str.push_str(&self.visit_nodes(rust, children)?);
			str.push_str(&self.visit_element_close(&tag, self_closing)?);
			Ok(str)
		} else {
			Err(ParseError::hydration("expected component"))
		}
	}

	fn visit_element_open(
		&mut self,
		rust: &mut Vec<RsxRust>,
		tag: &str,
		attributes: Vec<Attribute>,
		self_closing: bool,
	) -> ParseResult<String> {
		let mut out = String::new();
		out.push_str(&format!("<{}", tag));
		for attribute in attributes {
			out.push(' ');
			out.push_str(&self.visit_attribute(rust, attribute)?);
		}
		if self_closing {
			out.push_str("/>");
		} else {
			out.push('>');
		}
		Ok(out)
	}


	fn visit_element_close(
		&mut self,
		tag: &str,
		self_closing: bool,
	) -> ParseResult<String> {
		if self_closing {
			Ok("".to_string())
		} else {
			Ok(format!("</{}>", tag))
		}
	}


	fn visit_attribute(
		&mut self,
		rust: &mut Vec<RsxRust>,
		attribute: Attribute,
	) -> ParseResult<String> {
		match attribute {
			Attribute::Key { key } => Ok(key),
			Attribute::KeyValue { key, value } => {
				Ok(format!("{}=\"{}\"", key, value))
			}
			Attribute::BlockValue { key } => {
				if let Some(RsxRust::AttributeValue(val)) = rust.pop() {
					Ok(format!("{}=\"{}\"", key, val))
				} else {
					Err(ParseError::hydration("expected attribute value"))
				}
			}
			Attribute::Block => {
				if let Some(RsxRust::AttributeKey(key)) = rust.pop() {
					Ok(key)
				} else {
					Err(ParseError::hydration("expected attribute key"))
				}
			}
		}
	}
}


#[cfg(test)]
mod test {
	// use super::SweetRenderPlugin;
	// use crate::render::RenderPlugin;
	use sweet::prelude::*;

	#[test]
	fn works() {
		// let onclick = |_| {};
		let world = "mars";
		let rsx = rsx! {
			<div>
				<p>hello {world}</p>
			</div>
		};

		println!("rsx: '{:?}'", rsx);

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
