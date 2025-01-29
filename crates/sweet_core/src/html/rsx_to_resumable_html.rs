use crate::prelude::*;





#[derive(Default)]
pub struct RsxToResumableHtml {
	/// add attributes required for resumability
	pub html_constants: HtmlConstants,
	/// tracking this allows us to match with [RsxContext]
	pub num_html_elements: usize,
}
impl RsxToResumableHtml {
	pub fn render(node: &RsxNode) -> String {
		Self::default().map_node(node).render()
	}

	pub fn map_node(&mut self, node: &RsxNode) -> Vec<HtmlNode> {
		let mut html = RsxToHtml::for_resumable().map_node(node);
		for node in html.iter_mut() {
			self.visit_node(node);
		}

		self.insert_rsx_context_map(node, &mut html);
		self.insert_catch_prehydrated_events(&mut html);
		html
	}

	fn visit_node(&mut self, node: &mut HtmlNode) {
		match node {
			HtmlNode::Element(el) => {
				for attr in el.attributes.iter_mut() {
					if attr.key == "needs-id" {
						attr.key = self.html_constants.id_key.to_string();
						attr.value = Some(self.num_html_elements.to_string());
					} else if attr.key.starts_with("on") {
						attr.value = Some(format!(
							"{}({}, event)",
							self.html_constants.event_handler,
							self.num_html_elements,
						));
					}
				}

				self.num_html_elements += 1;


				for child in &mut el.children {
					self.visit_node(child);
				}
			}
			_ => {}
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
				key: self.html_constants.cx_map_key.to_string(),
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
	use sweet_test::prelude::*;

	#[test]
	fn plain() {
		expect(RsxToResumableHtml::render(&rsx! { <br/> }))
			.to_start_with("<br/>");
	}
	#[test]
	fn id() {
		expect(RsxToResumableHtml::render(
			&rsx! { <main><article>{7}</article></main> },
		))
		.to_start_with("<main><article data-sweet-id=\"1\">7</article></main>");
	}
	#[test]
	fn events() {
		let on_click = |_| {};

		expect(RsxToResumableHtml::render(
			&rsx! { <main onclick=on_click></main> },
		))
		.to_start_with("<main onclick=\"_sweet_event(0, event)\"></main>");
	}
}
