use crate::prelude::*;
use std::collections::VecDeque;
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
pub struct RsxRenderer<'a, V> {
	visitor: &'a mut V,
	/// The collection of rust blocks,
	/// the strings are used for initial values and events
	/// are discarded (used by the sweet loader)
	rust: VecDeque<RsxRust>,
}

impl<'a, V: RsxVisitor> RsxRenderer<'a, V> {
	/// Render [RsxParts] into a html string, returning the modified html partial
	pub fn render(
		visitor: &'a mut V,
		rsx: impl Rsx,
	) -> ParseResult<(String, HtmlPartial)> {
		let RsxParts { rust, html } = rsx.into_parts();
		let mut html = html.load()?;
		let mut renderer = Self { visitor, rust };
		let html_str = renderer.render_nodes(&mut html.nodes)?;

		if renderer.rust.len() != 0 {
			return Err(ParseError::Hydration(format!(
				"Unused rust blocks: {}",
				renderer
					.rust
					.iter()
					.map(|r| r.as_ref())
					.collect::<Vec<_>>()
					.join(", ")
			)));
		}
		Ok((html_str, html))
	}

	/// The render function will parse the parent node
	/// depth-first traversal of children,
	/// incrementing id

	fn render_nodes(&mut self, nodes: &mut Vec<Node>) -> ParseResult<String> {
		let mut out = String::new();
		for node in nodes {
			out.push_str(&self.render_node(node)?);
		}
		Ok(out)
	}

	fn render_node(&mut self, node: &mut Node) -> ParseResult<String> {
		self.visitor.visit_node(node)?;
		match node {
			Node::Doctype => Ok("<!DOCTYPE html>".to_string()),
			Node::Comment(val) => Ok(format!("<!-- {} -->", val)),
			Node::Text(val) => Ok(val.clone()),
			Node::Element(el) => {
				self.visitor.visit_element(el)?;
				let mut str = self.render_element_open(el)?;
				str.push_str(&self.render_nodes(&mut el.children)?);
				str.push_str(&self.render_element_close(el)?);
				Ok(str)
			}
			Node::TextBlock => {
				if let RsxRust::InnerText(val) = self.get_rust()? {
					Ok(val.clone())
				} else {
					Err(ParseError::hydration("expected text block", "block"))
				}
			}
			Node::Component(children) => self.render_component(children),
		}
	}
	/// 1.
	fn render_component(
		&mut self,
		children: &mut Vec<Node>,
	) -> ParseResult<String> {
		match self.get_rust()? {
			RsxRust::Component(component) => {
				// render 'passed in' children first
				let mut str = self.render_nodes(children)?;
				let (component_children_str, component_children) =
					RsxRenderer::render(self.visitor, component)?;
				str.push_str(&component_children_str);
				children.extend(component_children.nodes);
				Ok(str)
			}
			other => Err(ParseError::hydration("Component", &other)),
		}
	}

	fn render_element_open(&mut self, el: &mut Element) -> ParseResult<String> {
		let mut out = String::new();

		out.push_str(&format!("<{}", el.tag));
		for attribute in &mut el.attributes {
			out.push(' ');
			out.push_str(&self.render_attribute(attribute)?);
		}
		if el.self_closing {
			out.push_str("/>");
		} else {
			out.push('>');
		}
		Ok(out)
	}


	fn render_element_close(&mut self, el: &Element) -> ParseResult<String> {
		if el.self_closing {
			Ok("".to_string())
		} else {
			Ok(format!("</{}>", el.tag))
		}
	}


	fn render_attribute(
		&mut self,
		attribute: &mut Attribute,
	) -> ParseResult<String> {
		match attribute {
			Attribute::Key { key } => Ok(key.clone()),
			Attribute::KeyValue { key, value } => {
				Ok(format!("{}=\"{}\"", key, value))
			}
			Attribute::BlockValue { key } => {
				let is_event = key.starts_with("on");
				match (is_event, self.get_rust()?) {
					(true, RsxRust::Event(_)) => {
						let mut value = String::from("placeholder");
						self.visitor.visit_event_attribute(key, &mut value)?;
						Ok(format!("{}=\"{}\"", key, value))
					}
					(true, val) => {
						Err(ParseError::hydration("RsxRustEvent", val))
					}
					(false, RsxRust::AttributeValue(val)) => {
						Ok(format!("{}=\"{}\"", key, val))
					}
					(false, val) => Err(ParseError::hydration(
						"RsxRust::AttributeValue",
						val,
					)),
				}
			}
			Attribute::Block => {
				if let RsxRust::AttributeKey(key) = self.get_rust()? {
					Ok(key)
				} else {
					Err(ParseError::hydration(
						"expected attribute key",
						"block",
					))
				}
			}
		}
	}
	fn get_rust(&mut self) -> ParseResult<RsxRust> {
		if let Some(mut rust) = self.rust.pop_front() {
			self.visitor.visit_rust(&mut rust)?;
			Ok(rust)
		} else {
			Err(ParseError::Hydration(format!("Too few rust blocks")))
		}
	}
}


#[cfg(test)]
mod test {
	// use super::SweetRenderPlugin;
	// use crate::render::RenderPlugin;
	use crate::render::RsxRenderer;
	use crate::render::SweetRsxVisitor;
	use sweet::prelude::*;


	fn render(rsx: impl Rsx) -> (String, HtmlPartial) {
		let mut visitor = SweetRsxVisitor::default();
		RsxRenderer::render(&mut visitor, rsx).unwrap()
	}

	#[test]
	fn doctype() {
		let (str, _) = render(rsx! { <!DOCTYPE html> });
		expect(str).to_be("<!DOCTYPE html>");
	}

	#[test]
	fn comment() {
		let (str, _) = render(rsx! { <!-- "hello" --> });
		expect(str).to_be("<!-- hello -->");
	}

	#[test]
	fn text() {
		let (str, _) = render(rsx! { "hello" });
		expect(str).to_be("hello");
	}

	#[test]
	fn element() {
		let (str, _) = render(rsx! { <div></div> });
		expect(str).to_be("<div></div>");
	}
	#[test]
	fn element_self_closing() {
		let (str, _) = render(rsx! { <br/> });
		expect(str).to_be("<br/>");
	}
	#[test]
	fn element_children() {
		let (str, _) = render(rsx! { <div>hello</div> });
		expect(str).to_be("<div>hello</div>");
	}

	#[test]
	fn text_block() {
		let value = "hello";
		let (str, _) = render(rsx! { {value} });
		expect(str).to_be("hello");
	}

	#[test]
	fn component() {
		struct Child {
			value: u32,
		}
		impl Rsx for Child {
			fn into_parts(self) -> RsxParts {
				rsx! {
					<div>{self.value}</div>
				}
			}
		}
		let (str, _) = render(rsx! { <Child value=7/> });
		expect(str).to_be(
			"<div data-sweet-id=\"1\" data-sweet-blocks=\"0,0\">7</div>",
		);
	}


	#[test]
	fn nexted() {
		let onclick = |_| {};
		let world = "mars";
		let rsx = rsx! {
			<div onclick>
				<p>hello {world}</p>
			</div>
		};
		expect(format!("{:?}", rsx.rust)).to_be("[Event, InnerText]");

		println!("rsx: '{:#?}'", rsx);
		let (str, _) = render(rsx);
		// println!("rendered_tree: '{:#?}'", rendered_tree);
		// println!("html: '{}'", rendered_str);

		// expect(true).to_be_false();
	}
}
