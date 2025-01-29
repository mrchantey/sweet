use crate::prelude::*;

pub struct RsxToHtml {
	/// add attributes required for resumability
	pub resumable: bool,
	pub html_constants: HtmlConstants,
	/// tracking this allows us to match with [RsxContext]
	pub num_html_elements: usize,
}


impl Default for RsxToHtml {
	fn default() -> Self {
		Self {
			resumable: false,
			html_constants: Default::default(),
			num_html_elements: 0,
		}
	}
}


impl RsxToHtml {
	/// Render with default [IntoHtmlOptions]
	pub fn render(node: &RsxNode) -> String {
		Self::default().map_node(&node).render()
	}

	pub fn render_resumable(node: &RsxNode) -> String {
		let mut this = Self {
			resumable: true,
			..Default::default()
		};
		let mut html = this.map_node(node);
		this.insert_rsx_context_map(node, &mut html);
		this.insert_catch_prehydrated_events(&mut html);
		html.render()
	}

	pub fn map_node(&mut self, node: &RsxNode) -> Vec<HtmlNode> {
		match node {
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
				value: Some(self.num_html_elements.to_string()),
			});
		}
		self.num_html_elements += 1;

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


	/// attempt to insert the rsx context map into the html body,
	/// otherwise append it to the end of the html
	fn insert_rsx_context_map(
		&self,
		node: &RsxNode,
		nodes: &mut Vec<HtmlNode>,
	) {
		let rsx_context_map = RsxContextMap::from_node(node).to_csv();



		let el = HtmlElementNode::inline_script(rsx_context_map, vec![
			HtmlAttribute {
				key: self.html_constants.rsx_context_attribute_key.to_string(),
				value: None,
			},
		]);
		HtmlNode::insert_at_body_or_append(nodes, el.into());
	}

	fn insert_catch_prehydrated_events(&self, nodes: &mut Vec<HtmlNode>) {
		let event_handler = self.html_constants.event_handler;
		let prehydrate_events = self.html_constants.prehydrate_events;

		let script = format!(
			r#"
globalThis.{prehydrate_events} = []
globalThis.{event_handler} = (id,event) => globalThis.{prehydrate_events}.push([id, event])
"#
		);
		let el = HtmlElementNode::inline_script(script, Default::default());
		HtmlNode::insert_at_body_or_append(nodes, el.into());
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
	fn rsx_text() {
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
