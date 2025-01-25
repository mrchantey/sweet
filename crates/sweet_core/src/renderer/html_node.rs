pub enum HtmlNode {
	Doctype,
	Comment(HtmlCommentNode),
	Text(HtmlTextNode),
	Element(HtmlElementNode),
}

impl RenderHtml for HtmlNode {
	fn render_html_with_buf(&self, html: &mut String) {
		match self {
			HtmlNode::Doctype => html.push_str("<!DOCTYPE html>"),
			HtmlNode::Comment(node) => node.render_html_with_buf(html),
			HtmlNode::Text(node) => node.render_html_with_buf(html),
			HtmlNode::Element(node) => node.render_html_with_buf(html),
		}
	}
}

pub struct HtmlTextNode(pub String);

impl RenderHtml for HtmlTextNode {
	fn render_html_with_buf(&self, html: &mut String) {
		html.push_str(&self.0);
	}
}

pub struct HtmlCommentNode(pub String);
impl RenderHtml for HtmlCommentNode {
	fn render_html_with_buf(&self, html: &mut String) {
		html.push_str(&format!("<!--{}-->", self.0));
	}
}

pub struct HtmlElementNode {
	pub tag: String,
	pub self_closing: bool,
	pub attributes: Vec<HtmlAttribute>,
	pub children: Vec<HtmlNode>,
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

pub trait RenderHtml {
	fn render_html(&self) -> String {
		let mut html = String::new();
		self.render_html_with_buf(&mut html);
		html
	}

	fn render_html_with_buf(&self, html: &mut String);
}

impl RenderHtml for Vec<HtmlNode> {
	fn render_html_with_buf(&self, html: &mut String) {
		for node in self {
			node.render_html_with_buf(html);
		}
	}
}
