use sweet_core::prelude::*;

#[derive(Default)]
pub struct SweetRsxVisitor {
	constants: HtmlConstants,
	position: TreePosition,
}
impl SweetRsxVisitor {
	pub fn new(constants: HtmlConstants) -> Self {
		Self {
			constants,
			..Default::default()
		}
	}
	pub fn constants(&self) -> &HtmlConstants { &self.constants }
	pub fn position(&self) -> &TreePosition { &self.position }
}


#[derive(Debug, Clone)]
pub struct HtmlConstants {
	/// used for encoding the [TreePosition],
	pub id_attribute_key: &'static str,
	/// used for describing the location of rust blocks in text nodes,
	/// defaults to `data-sweet-blocks`
	pub block_attribute_key: &'static str,
	/// defaults to `_sweet_event`
	pub event_handler: &'static str,
}
impl HtmlConstants {
	pub const DEFAULT_ID_ATTRIBUTE_KEY: &'static str = "data-sweet-id";
	pub const DEFAULT_BLOCK_ATTRIBUTE_KEY: &'static str = "data-sweet-blocks";
	pub const DEFAULT_EVENT_HANDLER: &'static str = "_sweet_event";
}

impl Default for HtmlConstants {
	fn default() -> Self {
		Self {
			id_attribute_key: Self::DEFAULT_ID_ATTRIBUTE_KEY,
			block_attribute_key: Self::DEFAULT_BLOCK_ATTRIBUTE_KEY,
			event_handler: Self::DEFAULT_EVENT_HANDLER,
		}
	}
}

impl TreeMapper<RsxNode<RustParts>, RsxNode<String>> for SweetRsxVisitor {
	fn position(&mut self) -> &mut TreePosition { &mut self.position }

	fn map_node(
		&mut self,
		node: RsxNode<RustParts>,
	) -> ParseResult<RsxNode<String>> {
		let node = match node {
			RsxNode::Doctype => RsxNode::Doctype,
			RsxNode::Comment(val) => RsxNode::Comment(val),
			RsxNode::Element(el) => RsxNode::Element(self.parse_element(el)?),
			RsxNode::Text(val) => RsxNode::Text(val),
			RsxNode::TextBlock(RustParts::TextBlock(val)) => {
				RsxNode::TextBlock(val)
			}
			RsxNode::TextBlock(parts) => {
				return Err(ParseError::hydration("TextBlock", parts));
			}
			RsxNode::Component(RustParts::Component(component), children) => {
				// components arent html, return empty string
				let mut component_children = self.map_nodes(component.nodes)?;
				component_children.append(&mut self.map_nodes(children)?);
				RsxNode::Component(String::new(), component_children)
			}
			RsxNode::Component(parts, _) => {
				return Err(ParseError::hydration("Component", parts));
			}
		};
		Ok(node)
	}
}

impl SweetRsxVisitor {
	fn parse_element(
		&mut self,
		mut el: RsxElement<RustParts>,
	) -> ParseResult<RsxElement<String>> {
		if el.contains_rust() {
			el.attributes.push(RsxAttribute::KeyValue {
				key: self.constants.id_attribute_key.to_string(),
				// should this be full position or just index?
				value: (self.position.index()).to_string(),
			});
		}
		if el.contains_text_blocks() {
			el.attributes.push(RsxAttribute::KeyValue {
				key: self.constants.block_attribute_key.to_string(),
				value: TextBlockEncoder::encode(&el.children),
			});
		}
		let children = self.map_nodes(el.children)?;
		let attributes = el
			.attributes
			.into_iter()
			.map(|attr| self.parse_attribute(attr))
			.collect::<ParseResult<Vec<_>>>()?;

		Ok(RsxElement {
			tag: el.tag,
			self_closing: el.self_closing,
			attributes,
			children,
		})
	}


	fn parse_attribute(
		&self,
		attribute: RsxAttribute<RustParts>,
	) -> ParseResult<RsxAttribute<String>> {
		let attr = match attribute {
			RsxAttribute::Key { key } => RsxAttribute::Key { key },
			RsxAttribute::KeyValue { key, value } => {
				RsxAttribute::KeyValue { key, value }
			}
			RsxAttribute::BlockValue { key, value } => {
				let is_event = key.starts_with("on");
				let value = match (is_event, value) {
					(true, RustParts::Event(_)) => {
						self.constants.event_handler.to_string()
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
				RsxAttribute::BlockValue { key, value }
			}
			RsxAttribute::Block(RustParts::AttributeBlock(block_str)) => {
				RsxAttribute::Block(block_str)
			}
			RsxAttribute::Block(parts) => {
				return Err(ParseError::hydration(
					"RustParts::AttributeBlock",
					parts,
				))
			}
		};
		Ok(attr)
	}
}
