use sweet_core::prelude::*;

#[derive(Default)]
pub struct SweetRsxVisitor {
	pub position_visitor: RsxTreePositionVisitor,
}

impl SweetRsxVisitor {
	pub const ID_ATTRIBUTE_KEY: &str = "data-sweet-id";
	pub const BLOCK_ATTRIBUTE_KEY: &str = "data-sweet-blocks";
}

impl RsxTreeMapper<RustParts, String> for SweetRsxVisitor {
	fn map_nodes(
		&mut self,
		mut nodes: Vec<Node<RustParts>>,
	) -> ParseResult<Vec<Node<String>>> {
		self.position_visitor.visit_children(&mut nodes)?;
		let mut nodes = nodes
			.into_iter()
			.map(|node| self.map_node(node))
			.collect::<ParseResult<_>>()?;
		self.position_visitor.leave_children(&mut nodes)?;
		Ok(nodes)
	}


	fn map_node(
		&mut self,
		mut node: Node<RustParts>,
	) -> ParseResult<Node<String>> {
		self.position_visitor.visit_node(&mut node)?;
		let mut node = match node {
			Node::Doctype => Node::Doctype,
			Node::Comment(val) => Node::Comment(val),
			Node::Element(el) => Node::Element(self.parse_element(el)?),
			Node::Text(val) => Node::Text(val),
			Node::TextBlock(RustParts::TextBlock(val)) => Node::TextBlock(val),
			Node::TextBlock(parts) => {
				return Err(ParseError::hydration("TextBlock", parts));
			}
			Node::Component(RustParts::Component(component), children) => {
				// components arent html, return empty string
				let mut component_children = self.map_nodes(component.nodes)?;
				component_children.append(&mut self.map_nodes(children)?);
				Node::Component(String::new(), component_children)
			}
			Node::Component(parts, _) => {
				return Err(ParseError::hydration("Component", parts));
			}
		};
		self.position_visitor.leave_node(&mut node)?;
		Ok(node)
	}
}

impl SweetRsxVisitor {
	fn parse_element(
		&mut self,
		mut el: Element<RustParts>,
	) -> ParseResult<Element<String>> {
		if el.contains_rust() {
			el.attributes.push(Attribute::KeyValue {
				key: Self::ID_ATTRIBUTE_KEY.to_string(),
				value: (self.position_visitor.current_node_id()).to_string(),
			});
		}
		if el.contains_text_blocks() {
			el.attributes.push(Attribute::KeyValue {
				key: Self::BLOCK_ATTRIBUTE_KEY.to_string(),
				value: TextBlockEncoder::encode(&el.children),
			});
		}
		let children = self.map_nodes(el.children)?;
		let attributes = el
			.attributes
			.into_iter()
			.map(|attr| self.parse_attribute(attr))
			.collect::<ParseResult<Vec<_>>>()?;

		Ok(Element {
			tag: el.tag,
			self_closing: el.self_closing,
			attributes,
			children,
		})
	}


	fn parse_attribute(
		&self,
		attribute: Attribute<RustParts>,
	) -> ParseResult<Attribute<String>> {
		let attr = match attribute {
			Attribute::Key { key } => Attribute::Key { key },
			Attribute::KeyValue { key, value } => {
				Attribute::KeyValue { key, value }
			}
			Attribute::BlockValue { key, value } => {
				let is_event = key.starts_with("on");
				let value = match (is_event, value) {
					(true, RustParts::Event(_)) => {
						format!(
							"_sweet.event({},event)",
							self.position_visitor.current_node_id()
						)
					}
					(true, parts) => {
						return Err(ParseError::hydration(
							"RustParts::Event",
							parts,
						));
					}
					(false, RustParts::AttributeValue(val)) => val,
					(false, val) => {
						return Err(ParseError::hydration(
							"RustParts::AttributeValue",
							val,
						))
					}
				};
				Attribute::BlockValue { key, value }
			}
			Attribute::Block(RustParts::AttributeBlock(block_str)) => {
				Attribute::Block(block_str)
			}
			Attribute::Block(parts) => {
				return Err(ParseError::hydration(
					"RustParts::AttributeBlock",
					parts,
				))
			}
		};
		Ok(attr)
	}
}
