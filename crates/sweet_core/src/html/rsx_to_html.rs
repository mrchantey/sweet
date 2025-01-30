use crate::prelude::*;

pub struct RsxToHtml {
	/// on elements that have children, give them a `needs-id` attribute
	/// to be mapped by [RsxToResumableHtml]
	pub mark_needs_id: bool,
}


impl Default for RsxToHtml {
	fn default() -> Self {
		Self {
			mark_needs_id: false,
		}
	}
}


impl RsxToHtml {
	pub fn as_resumable() -> Self {
		Self {
			mark_needs_id: true,
		}
	}

	/// convenience so you dont have to add
	/// a `.render()` at the end of a long rsx macro
	pub fn render_body(node: &RsxNode) -> String {
		Self::default().map_node(node).render()
	}

	pub fn map_node(&mut self, rsx_node: &RsxNode) -> Vec<HtmlNode> {
		match rsx_node {
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
		}
	}

	pub fn map_element(&mut self, rsx_el: &RsxElement) -> HtmlElementNode {
		let mut html_attributes = rsx_el
			.attributes
			.iter()
			.map(|a| self.map_attribute(a))
			.flatten()
			.collect::<Vec<_>>();

		if self.mark_needs_id && rsx_el.contains_rust() {
			html_attributes.push(HtmlAttribute {
				key: "needs-id".to_string(),
				value: None,
			});
		}

		HtmlElementNode {
			tag: rsx_el.tag.clone(),
			self_closing: rsx_el.self_closing,
			attributes: html_attributes,
			children: rsx_el
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
		// HtmlRenderer::render_body_default
		expect(RsxToHtml::render_body(&rsx! { <!DOCTYPE html> }))
			.to_be("<!DOCTYPE html>");
	}

	#[test]
	fn comment() {
		expect(RsxToHtml::render_body(&rsx! { <!-- "hello" --> }))
			.to_be("<!-- hello -->");
	}

	#[test]
	fn text() {
		expect(RsxToHtml::render_body(&rsx! { "hello" })).to_be("hello");
	}

	#[test]
	fn element() {
		let _key = "hidden";
		let _key_value = "class=\"pretty\"";
		let food = "pizza";
		expect(RsxToHtml::render_body(&rsx! { <div
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
		expect(RsxToHtml::render_body(&rsx! { <br/> })).to_be("<br/>");
	}
	#[test]
	fn element_children() {
		expect(RsxToHtml::render_body(&rsx! { <div>hello</div> }))
			.to_be("<div>hello</div>");
	}

	#[test]
	fn rsx_text() {
		let value = "hello";
		expect(RsxToHtml::render_body(&rsx! { {value} })).to_be("hello");
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
		expect(RsxToHtml::render_body(&rsx! { <Child value=7/> }))
			.to_be("<div>7</div>");
	}


	#[test]
	fn nested() {
		let world = "mars";
		expect(RsxToHtml::render_body(&rsx! {
			<div>
				<p>hello {world}</p>
			</div>
		}))
		.to_be("<div><p>hello mars</p></div>");
	}
	#[test]
	fn events() {
		let onclick = |_| {};
		let world = "mars";
		expect(RsxToHtml::render_body(&rsx! {
			<div onclick=onclick>
				<p>hello {world}</p>
			</div>
		}))
		.to_be("<div onclick=\"needs-event-cx\"><p>hello mars</p></div>");
	}
}
