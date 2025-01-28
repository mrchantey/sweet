use crate::prelude::*;

pub struct RsxToHtml {
	/// add attributes required for resumability
	pub resumable: bool,
	/// For every html node visited, this will increment by 1.
	/// If resumable, the id will be attached to the html.
	pub cx: RsxContext,
	pub html_constants: HtmlConstants,
}


impl Default for RsxToHtml {
	fn default() -> Self {
		Self {
			resumable: false,
			html_constants: Default::default(),
			cx: Default::default(),
		}
	}
}


impl RsxToHtml {
	/// Render with default [IntoHtmlOptions]
	pub fn render(node: &RsxNode) -> String {
		Self::default().render_node(&node)
	}

	pub fn render_resumable(node: &RsxNode) -> String {
		Self {
			resumable: true,
			..Default::default()
		}
		.render_node(node)
	}


	/// Render with default [IntoHtmlOptions]
	pub fn render_node(&mut self, node: &RsxNode) -> String {
		self.map_node(node).render()
	}

	pub fn map_node(&mut self, node: &RsxNode) -> Vec<HtmlNode> {
		let val = match node {
			RsxNode::Doctype => {
				vec![HtmlNode::Doctype]
			}
			RsxNode::Comment(s) => {
				vec![HtmlNode::Comment(s.clone())]
			}
			RsxNode::Text(s) => {
				vec![HtmlNode::Text(s.clone())]
			}
			RsxNode::Block { initial, .. } => self.map_node(initial),
			RsxNode::Element(e) => {
				vec![HtmlNode::Element(self.map_element(e))]
			}
			RsxNode::Fragment(nodes) => {
				nodes.iter().map(|n| self.map_node(n)).flatten().collect()
			}
		};
		self.cx.after_visit_node(&node.into_discriminant());
		val
	}

	pub fn map_element(&mut self, el: &RsxElement) -> HtmlElementNode {
		let mut attributes = el
			.attributes
			.iter()
			.map(|a| self.map_attribute(a))
			.flatten()
			.collect::<Vec<_>>();

		if self.resumable && el.contains_blocks() {
			attributes.push(HtmlAttribute {
				key: self.html_constants.id_attribute_key.to_string(),
				value: Some(self.cx.html_element_index().to_string()),
			});
		}

		HtmlElementNode {
			tag: el.tag.clone(),
			self_closing: el.self_closing,
			attributes,
			children: el
				.children
				.iter()
				.map(|c| self.map_node(c))
				.flatten()
				.collect(),
		}
	}

	pub fn map_attribute(&self, attr: &RsxAttribute) -> Vec<HtmlAttribute> {
		match attr {
			RsxAttribute::Key { key } => vec![HtmlAttribute {
				key: key.clone(),
				value: None,
			}],
			RsxAttribute::KeyValue { key, value } => vec![HtmlAttribute {
				key: key.clone(),
				value: Some(value.clone()),
			}],
			RsxAttribute::BlockValue { key, initial, .. } => {
				vec![HtmlAttribute {
					key: key.clone(),
					value: Some(initial.clone()),
				}]
			}
			RsxAttribute::Block { initial, .. } => initial
				.iter()
				.map(|a| self.map_attribute(a))
				.flatten()
				.collect(),
		}
	}
}



#[cfg(test)]
mod test {
	use crate::prelude::*;
	use sweet_rsx_macros::rsx;

	#[test]
	fn doctype() {
		// HtmlRenderer::render_default
		expect(RsxToHtml::render(&rsx! { <!DOCTYPE html> }))
			.to_be("<!DOCTYPE html>");
	}

	#[test]
	fn comment() {
		expect(RsxToHtml::render(&rsx! { <!-- "hello" --> }))
			.to_be("<!-- hello -->");
	}

	#[test]
	fn text() { expect(RsxToHtml::render(&rsx! { "hello" })).to_be("hello"); }

	#[test]
	fn element() {
		let _key = "hidden";
		let _key_value = "class=\"pretty\"";
		let food = "pizza";
		expect(RsxToHtml::render(&rsx! { <div
			name="pete"
			age=9
			// {key}
			// {key_value}
			favorite_food={food}
			>
			</div>
		}))
		.to_be("<div name=\"pete\" age=\"9\" favorite_food=\"pizza\"></div>");
	}
	#[test]
	fn element_self_closing() {
		expect(RsxToHtml::render(&rsx! { <br/> })).to_be("<br/>");
	}
	#[test]
	fn element_children() {
		expect(RsxToHtml::render(&rsx! { <div>hello</div> }))
			.to_be("<div>hello</div>");
	}

	#[test]
	fn text_block() {
		let value = "hello";
		expect(RsxToHtml::render(&rsx! { {value} })).to_be("hello");
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
		expect(RsxToHtml::render(&rsx! { <Child value=7/> }))
			.to_be("<div>7</div>");
	}


	#[test]
	fn nested() {
		let onclick = |_: usize| {};
		let world = "mars";
		expect(RsxToHtml::render(&rsx! {
			<div onclick=onclick>
				<p>hello {world}</p>
			</div>
		}))
		.to_be("<div onclick=\"onclick_handler\"><p>hello mars</p></div>");
	}
}
