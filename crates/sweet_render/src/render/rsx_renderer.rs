use super::SweetRsxVisitor;
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


pub struct DefaultRsxRenderer;

impl DefaultRsxRenderer {
	pub fn render(rsx: impl Rsx) -> ParseResult<String> {
		// 1. parser converts to rsx tree
		let rsx_tree = rsx.into_rsx_tree();
		// 2. sweet visitor converts rusty parts to valid html
		let mut html_nodes =
			SweetRsxVisitor::default().map_nodes(rsx_tree.nodes)?;
		// 3. render visitor flattens tree into html string
		let mut render_visitor = RsxRenderVisitor::default();
		render_visitor.walk_nodes_dfs(&mut html_nodes)?;
		Ok(render_visitor.html)
	}
}

#[derive(Default)]
pub struct RsxRenderVisitor {
	html: String,
}
impl RsxTreeVisitor<String> for RsxRenderVisitor {
	fn visit_node(&mut self, node: &Node<String>) -> ParseResult<()> {
		match node {
			Node::Doctype => self.html += "<!DOCTYPE html>",
			Node::Comment(val) => {
				self.html += "<!-- ";
				self.html += val;
				self.html += " -->";
			}
			Node::Text(val) => self.html += val,
			Node::Element(el) => {
				self.html += "<";
				self.html += &el.tag;
				for attribute in &el.attributes {
					self.html.push(' ');
					match attribute {
						Attribute::Key { key } => self.html += key,
						Attribute::KeyValue { key, value } => {
							self.html += &format!("{}=\"{}\"", key, value)
						}
						Attribute::BlockValue { key, value } => {
							self.html += &format!("{}=\"{}\"", key, value)
						}
						Attribute::Block(block) => self.html += block,
					};
				}
				if el.self_closing {
					self.html.push_str("/>");
				} else {
					self.html.push('>');
				}
			}
			Node::TextBlock(val) => {self.html += val},
			Node::Component(_, _) => {
				// components are not html
			}
		};
		Ok(())
	}
	fn leave_node(&mut self, node: &Node<String>) -> ParseResult<()> {
		match node {
			Node::Element(element) => {
				if !element.self_closing {
					self.html += &format!("</{}>", element.tag);
				} else {
				}
			}
			_ => {}
		}
		Ok(())
	}
}

#[cfg(test)]
mod test {
	use crate::prelude::*;
	use sweet_core as sweet;
	use sweet_core::prelude::*;
	use sweet_rsx_macros::rsx;
	use sweet_test::prelude::*;

	#[test]
	fn doctype() {
		let out = DefaultRsxRenderer::render(rsx! { <!DOCTYPE html> }).unwrap();
		expect(out).to_be("<!DOCTYPE html>");
	}

	#[test]
	fn comment() {
		let out =
			DefaultRsxRenderer::render(rsx! { <!-- "hello" --> }).unwrap();
		expect(out).to_be("<!-- hello -->");
	}

	#[test]
	fn text() {
		let out = DefaultRsxRenderer::render(rsx! { "hello" }).unwrap();
		expect(out).to_be("hello");
	}

	#[test]
	fn element() {
		let key = "hidden";
		let key_value = "class=\"pretty\"";
		let food = "pizza";
		let out = DefaultRsxRenderer::render(rsx! { <div
			name="pete"
			age=9
			{key}
			{key_value}
			favorite_food={food}
			>
			</div>
		})
		.unwrap();
		expect(out).to_be("<div name=\"pete\" age=\"9\" hidden class=\"pretty\" favorite_food=\"pizza\" data-sweet-id=\"0\"></div>");
	}
	#[test]
	fn element_self_closing() {
		let out = DefaultRsxRenderer::render(rsx! { <br/> }).unwrap();
		expect(out).to_be("<br/>");
	}
	#[test]
	fn element_children() {
		let out =
			DefaultRsxRenderer::render(rsx! { <div>hello</div> }).unwrap();
		expect(out).to_be("<div>hello</div>");
	}

	#[test]
	fn text_block() {
		let value = "hello";
		let out = DefaultRsxRenderer::render(rsx! { {value} }).unwrap();
		expect(out).to_be("hello");
	}

	#[test]
	fn component() {
		struct Child {
			value: u32,
		}
		impl Component for Child {
			fn render(self) -> impl Rsx {
				rsx! {
					<div>{self.value}</div>
				}
			}
		}
		let out =
			DefaultRsxRenderer::render(rsx! { <Child value=7/> }).unwrap();
		expect(out).to_be(
			"<div data-sweet-id=\"1\" data-sweet-blocks=\"0-0-1\">7</div>",
		);
	}


	#[test]
	fn nested() {
		let onclick = |_| {};
		let world = "mars";
		let rsx = rsx! {
			<div onclick>
				<p>hello {world}</p>
			</div>
		};
		// println!("rsx: '{:#?}'", rsx);
		let out = DefaultRsxRenderer::render(rsx).unwrap();
		expect(out).to_be("<div onclick=\"_sweet.event(0,event)\" data-sweet-id=\"0\"><p data-sweet-id=\"1\" data-sweet-blocks=\"0-6-4\">hello mars</p></div>");
	}
}
