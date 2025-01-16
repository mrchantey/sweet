use crate::prelude::*;
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
}

impl<'a, V: RsxVisitor + Default> RsxRenderer<'a, V> {
	pub fn render(rsx: impl Rsx) -> ParseResult<String> {
		let mut visitor = V::default();
		let mut tree = rsx.into_rsx_tree();
		RsxRenderer::render_with_visitor(&mut visitor, &mut tree)
	}
}

impl<'a, V: RsxVisitor> RsxRenderer<'a, V> {
	/// Render [RsxTree] into a html string, returning the modified html partial
	pub fn render_with_visitor(
		visitor: &'a mut V,
		tree: &mut RsxTree<RustParts>,
	) -> ParseResult<String> {
		let mut renderer = Self { visitor };
		let out = renderer.render_nodes(&mut tree.nodes)?;
		Ok(out)
	}

	/// The render function will parse the parent node
	/// depth-first traversal of children,
	/// incrementing id
	fn render_nodes(
		&mut self,
		nodes: &mut Vec<Node<RustParts>>,
	) -> ParseResult<String> {
		let mut out = String::new();
		for node in nodes {
			out.push_str(&self.render_node(node)?);
		}
		Ok(out)
	}

	fn render_node(
		&mut self,
		node: &mut Node<RustParts>,
	) -> ParseResult<String> {
		self.visitor.visit_node(node)?;
		match node {
			Node::Doctype => Ok("<!DOCTYPE html>".to_string()),
			Node::Comment(val) => Ok(format!("<!-- {} -->", val)),
			Node::Text(val) => Ok(val.clone()),
			Node::Element(el) => {
				// give visitor a chance to modify the element
				self.visitor.visit_element(el)?;
				let mut str = self.render_element_open(el)?;
				str.push_str(&self.render_nodes(&mut el.children)?);
				str.push_str(&self.render_element_close(el)?);
				Ok(str)
			}
			Node::TextBlock(RustParts::TextBlock(val)) => Ok(val.clone()),
			Node::TextBlock(other) => {
				Err(ParseError::hydration("RustParts::TextBlock", other))
			}
			Node::Component(RustParts::Component(component), children) => {
				self.render_component(component, children)
			}
			Node::Component(other, _) => {
				Err(ParseError::hydration("RustParts::Component", other))
			}
		}
	}
	fn render_component(
		&mut self,
		component_tree: &mut RsxTree<RustParts>,
		children: &mut Vec<Node<RustParts>>,
	) -> ParseResult<String> {
		// render 'passed in' children first
		let mut str = self.render_nodes(children)?;
		let child_out =
			RsxRenderer::render_with_visitor(self.visitor, component_tree)?;
		str.push_str(&child_out);
		Ok(str)
	}

	fn render_element_open(
		&mut self,
		el: &mut Element<RustParts>,
	) -> ParseResult<String> {
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


	fn render_element_close(
		&mut self,
		el: &Element<RustParts>,
	) -> ParseResult<String> {
		if el.self_closing {
			Ok("".to_string())
		} else {
			Ok(format!("</{}>", el.tag))
		}
	}


	fn render_attribute(
		&mut self,
		attribute: &mut Attribute<RustParts>,
	) -> ParseResult<String> {
		match attribute {
			Attribute::Key { key } => Ok(key.clone()),
			Attribute::KeyValue { key, value } => {
				// string literals are already quoted
				Ok(format!("{}=\"{}\"", key, value))
			}
			Attribute::BlockValue { key, value } => {
				let is_event = key.starts_with("on");
				match (is_event, value) {
					(true, RustParts::Event(_)) => {
						let value = self.visitor.visit_event_attribute(key)?;
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
			Attribute::Block(RustParts::AttributeBlock(block_str)) => {
				Ok(block_str.to_string())
			}
			Attribute::Block(other) => {
				Err(ParseError::hydration("RustParts::AttributeBlock", other))
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
		impl Rsx for Child {
			fn into_rsx_tree(self) -> RsxTree<RustParts> {
				rsx! {
					<div>{self.value}</div>
				}
			}
		}
		let out =
			DefaultRsxRenderer::render(rsx! { <Child value=7/> }).unwrap();
		expect(out).to_be(
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
		expect(out).to_be("<div onclick=\"_sweet.event(0,event)\" data-sweet-id=\"0\"><p data-sweet-id=\"1\" data-sweet-blocks=\"0,6\">hello mars</p></div>");
	}
}
