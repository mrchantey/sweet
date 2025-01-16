use crate::prelude::*;
use std::collections::VecDeque;
use sweet_core::prelude::*;


pub type DefaultRsxRenderer = RsxRenderer<'static, SweetRsxVisitor>;

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
	/// The collection of rust parts,
	/// the strings are used for initial values and events
	/// are discarded (used by the sweet loader)
	rust: VecDeque<RustParts>,
}

pub struct RsxRendererOut {
	pub html: HtmlPartial,
	pub html_str: String,
}

impl RsxRendererOut {
	pub fn new(html: HtmlPartial) -> Self {
		Self {
			html,
			html_str: String::new(),
		}
	}
}

impl<'a, V: RsxVisitor + Default> RsxRenderer<'a, V> {
	pub fn render(rsx: impl Rsx) -> ParseResult<RsxRendererOut> {
		let mut visitor = V::default();
		RsxRenderer::render_with_visitor(&mut visitor, rsx)
	}
}

impl<'a, V: RsxVisitor> RsxRenderer<'a, V> {
	/// Render [RsxParts] into a html string, returning the modified html partial
	pub fn render_with_visitor(
		visitor: &'a mut V,
		rsx: impl Rsx,
	) -> ParseResult<RsxRendererOut> {
		let RsxParts { rust, html } = rsx.into_rsx_parts();
		let html = html.load()?;
		let mut out = RsxRendererOut::new(html);

		let mut renderer = Self { visitor, rust };
		out.html_str = renderer.render_nodes(&mut out.html.nodes)?;

		if renderer.rust.len() != 0 {
			return Err(ParseError::Hydration(format!(
				"Unused rust parts: {}",
				renderer
					.rust
					.iter()
					.map(|r| r.as_ref())
					.collect::<Vec<_>>()
					.join(", ")
			)));
		}
		renderer.visitor.visit_final(&mut out)?;
		Ok(out)
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
				if let RustParts::InnerText(val) = self.get_rust()? {
					Ok(val.clone())
				} else {
					Err(ParseError::hydration("expected text block", "block"))
				}
			}
			Node::Component(children) => self.render_component(children),
		}
	}
	fn render_component(
		&mut self,
		children: &mut Vec<Node>,
	) -> ParseResult<String> {
		match self.get_rust()? {
			RustParts::Component(component) => {
				// render 'passed in' children first
				let mut str = self.render_nodes(children)?;
				let child_out =
					RsxRenderer::render_with_visitor(self.visitor, component)?;
				str.push_str(&child_out.html_str);
				children.extend(child_out.html.nodes);
				// self.total_rust_parts += child_out.num_rust_parts;
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
				// string literals are already quoted
				Ok(format!("{}=\"{}\"", key, value))
			}
			Attribute::BlockValue { key } => {
				let is_event = key.starts_with("on");
				match (is_event, self.get_rust()?) {
					(true, RustParts::Event(_)) => {
						let mut value = String::from("placeholder");
						self.visitor.visit_event_attribute(key, &mut value)?;
						Ok(format!("{}=\"{}\"", key, value))
					}
					(true, val) => {
						Err(ParseError::hydration("RustParts::Event", val))
					}
					(false, RustParts::AttributeValue(val)) => {
						Ok(format!("{}=\"{}\"", key, val))
					}
					(false, val) => Err(ParseError::hydration(
						"RustParts::AttributeValue",
						val,
					)),
				}
			}
			Attribute::Block => {
				if let RustParts::AttributeBlock(key) = self.get_rust()? {
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
	fn get_rust(&mut self) -> ParseResult<RustParts> {
		if let Some(mut rust) = self.rust.pop_front() {
			self.visitor.visit_rust(&mut rust)?;
			Ok(rust)
		} else {
			Err(ParseError::Hydration(format!("Too few rust parts")))
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

	#[test]
	fn doctype() {
		let out = DefaultRsxRenderer::render(rsx! { <!DOCTYPE html> }).unwrap();
		expect(out.html_str).to_be("<!DOCTYPE html>");
	}

	#[test]
	fn comment() {
		let out =
			DefaultRsxRenderer::render(rsx! { <!-- "hello" --> }).unwrap();
		expect(out.html_str).to_be("<!-- hello -->");
	}

	#[test]
	fn text() {
		let out = DefaultRsxRenderer::render(rsx! { "hello" }).unwrap();
		expect(out.html_str).to_be("hello");
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
		expect(out.html_str).to_be("<div name=\"pete\" age=\"9\" hidden class=\"pretty\" favorite_food=\"pizza\" data-sweet-id=\"0\"></div>");
	}
	#[test]
	fn element_self_closing() {
		let out = DefaultRsxRenderer::render(rsx! { <br/> }).unwrap();
		expect(out.html_str).to_be("<br/>");
	}
	#[test]
	fn element_children() {
		let out =
			DefaultRsxRenderer::render(rsx! { <div>hello</div> }).unwrap();
		expect(out.html_str).to_be("<div>hello</div>");
	}

	#[test]
	fn text_block() {
		let value = "hello";
		let out = DefaultRsxRenderer::render(rsx! { {value} }).unwrap();
		expect(out.html_str).to_be("hello");
	}

	#[test]
	fn component() {
		struct Child {
			value: u32,
		}
		impl Rsx for Child {
			fn into_rsx_parts(self) -> RsxParts {
				rsx! {
					<div>{self.value}</div>
				}
			}
		}
		let out =
			DefaultRsxRenderer::render(rsx! { <Child value=7/> }).unwrap();
		expect(out.html_str).to_be(
			"<div data-sweet-id=\"0\" data-sweet-blocks=\"0,0\">7</div>",
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
		expect(out.html_str).to_be("<div onclick=\"_sweet.event(0,event)\" data-sweet-id=\"0\"><p data-sweet-id=\"1\" data-sweet-blocks=\"0,6\">hello mars</p></div>");
	}
}
