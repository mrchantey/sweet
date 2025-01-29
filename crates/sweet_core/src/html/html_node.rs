/// A trait for rendering a value to HTML
pub trait RenderHtml {
	/// Convert a value, usually [HtmlNode] to a string of HTML
	fn render(&self) -> String {
		let mut html = String::new();
		self.render_html_with_buf(&mut html);
		html
	}

	fn render_html_with_buf(&self, html: &mut String);
}

/// Unlike RsxNode, this struct contains only real html nodes
#[derive(Debug, Clone)]
pub enum HtmlNode {
	Doctype,
	Comment(String),
	Text(String),
	Element(HtmlElementNode),
}

impl HtmlNode {
	/// recursively search for an html node with a matching id
	pub fn query_selector_attr(
		&mut self,
		key: &str,
		val: Option<&str>,
	) -> Option<&mut HtmlElementNode> {
		match self {
			HtmlNode::Element(e) => {
				if e.query_selector_attr(key, val) {
					return Some(e);
				}
				for child in &mut e.children {
					if let Some(node) = child.query_selector_attr(key, val) {
						return Some(node);
					}
				}
			}
			_ => {}
		}
		None
	}

	/// return self as an element if it matches the tag
	pub fn element_with_tag(
		&mut self,
		tag: &str,
	) -> Option<&mut HtmlElementNode> {
		match self {
			HtmlNode::Element(e) => {
				if e.tag == tag {
					return Some(e);
				}
			}
			_ => {}
		}
		None
	}

	/// attempt to find `<html><body>` and insert the node into the body,
	/// otherwise append the node to the root
	pub fn insert_at_body_or_append(nodes: &mut Vec<Self>, node: HtmlNode) {
		if let Some(html_root) =
			nodes.iter_mut().find_map(|e| e.element_with_tag("html"))
		{
			if let Some(body) = html_root
				.children
				.iter_mut()
				.find_map(|e| e.element_with_tag("body"))
			{
				body.children.push(node);
				return;
			}
		}
		nodes.push(node);
	}
}


impl RenderHtml for Vec<HtmlNode> {
	fn render_html_with_buf(&self, html: &mut String) {
		for node in self {
			node.render_html_with_buf(html);
		}
	}
}

impl RenderHtml for HtmlNode {
	fn render_html_with_buf(&self, html: &mut String) {
		match self {
			HtmlNode::Doctype => html.push_str("<!DOCTYPE html>"),
			HtmlNode::Comment(val) => {
				html.push_str(&format!("<!-- {} -->", val))
			}
			HtmlNode::Text(val) => html.push_str(val),
			HtmlNode::Element(node) => node.render_html_with_buf(html),
		}
	}
}
#[derive(Debug, Clone)]
pub struct HtmlElementNode {
	pub tag: String,
	pub self_closing: bool,
	pub attributes: Vec<HtmlAttribute>,
	pub children: Vec<HtmlNode>,
}



impl Into<HtmlNode> for HtmlElementNode {
	fn into(self) -> HtmlNode { HtmlNode::Element(self) }
}

impl HtmlElementNode {
	pub fn inline_script(
		script: String,
		attributes: Vec<HtmlAttribute>,
	) -> Self {
		Self {
			tag: "script".to_string(),
			self_closing: false,
			attributes,
			children: vec![HtmlNode::Text(script)],
		}
	}


	/// returns true if any attribute matches the key and value
	pub fn query_selector_attr(
		&mut self,
		key: &str,
		val: Option<&str>,
	) -> bool {
		self.attributes
			.iter()
			.any(|a| a.key == key && a.value.as_deref() == val)
	}

	/// returns none if the attribute is not found or it has no value
	pub fn get_attribute_value(&self, key: &str) -> Option<&str> {
		for attr in &self.attributes {
			if attr.key == key {
				return attr.value.as_deref();
			}
		}
		None
	}
}

impl RenderHtml for HtmlElementNode {
	fn render_html_with_buf(&self, html: &mut String) {
		// slots are a kind of fragment, just return children
		if self.tag == "slot" {
			for child in &self.children {
				child.render_html_with_buf(html);
			}
			return;
		}

		html.push_str(&format!("<{}", self.tag));
		for attribute in &self.attributes {
			attribute.render_html_with_buf(html);
		}

		if self.self_closing {
			assert!(
				self.children.is_empty(),
				"self closing elements should not have children"
			);
			html.push_str("/>");
			return;
		} else {
			html.push('>');
		}
		for child in &self.children {
			child.render_html_with_buf(html);
		}
		html.push_str(&format!("</{}>", self.tag));
	}
}
#[derive(Debug, Clone)]
pub struct HtmlAttribute {
	pub key: String,
	pub value: Option<String>,
}


impl RenderHtml for HtmlAttribute {
	fn render_html_with_buf(&self, html: &mut String) {
		if self.key == "slot" {
			// slot attributes are for initial rendering
			return;
		}

		html.push(' ');
		html.push_str(&self.key);
		if let Some(value) = &self.value {
			html.push_str("=\"");
			html.push_str(value);
			html.push_str("\"");
		}
	}
}


impl RenderHtml for Vec<HtmlAttribute> {
	fn render_html_with_buf(&self, html: &mut String) {
		for attr in self {
			attr.render_html_with_buf(html);
		}
	}
}





#[derive(Debug, Clone)]
pub struct HtmlConstants {
	/// used for encoding the [TreePosition],
	pub id_attribute_key: &'static str,
	/// used for describing the location of rust blocks in text nodes,
	/// defaults to `data-sweet-blocks`
	pub rsx_context_attribute_key: &'static str,
	/// defaults to `_sweet_event`
	pub event_handler: &'static str,
	/// defaults to `_sweet_prehydrate_events`
	pub prehydrate_events: &'static str,
}

impl Default for HtmlConstants {
	fn default() -> Self {
		Self {
			id_attribute_key: "data-sweet-id",
			rsx_context_attribute_key: "data-sweet-blocks",
			event_handler: "_sweet_event",
			prehydrate_events: "_sweet_prehydrate_events",
		}
	}
}
