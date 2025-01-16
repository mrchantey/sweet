use sweet_core::prelude::*;

#[derive(Default)]
pub struct SweetRsxVisitor {
	pub position_visitor: RsxTreePositionVisitor,
	pub out: RsxTree<String>,
}

impl RsxTreeVisitor<RustParts> for SweetRsxVisitor {
	fn visit_node(&mut self, node: &mut Node<RustParts>) -> ParseResult<()> {
		self.position_visitor.visit_node(node)?;
		self.out.nodes.push(match node {
			Node::Doctype => Node::Doctype,
			Node::Comment(val) => Node::Comment(std::mem::take(val)),
			Node::Element(Element {
				tag,
				attributes,
				children,
				self_closing,
			}) => Node::Element(Element {
				tag: std::mem::take(tag),
				self_closing: *self_closing,
				attributes: attributes
					.into_iter()
					.map(|attr| self.parse_attribute(attr))
					.collect::<ParseResult<_>>()?,
				children: Vec::new(),
			}),
			Node::Text(val) => Node::Text(std::mem::take(val)),
			Node::TextBlock(RustParts::TextBlock(val)) => {
				Node::TextBlock(std::mem::take(val))
			}
			Node::TextBlock(parts) => {
				return Err(ParseError::hydration("TextBlock", parts));
			}
			Node::Component(RustParts::Component(_), children) => {
				
				// children are pushed in the visit children step?
				Node::Component(String::new(), Vec::new())
			}
			Node::Component(parts, _) => {
				return Err(ParseError::hydration("Component", parts));
			}
		});
		Ok(())
	}
	
	fn leave_node(&mut self, node: &mut Node<RustParts>) -> ParseResult<()> {
		self.position_visitor.leave_node(node)
	}
	fn visit_children(
		&mut self,
		children: &mut Vec<Node<RustParts>>,
	) -> ParseResult<()> {
		self.position_visitor.visit_children(children)
	}
	fn leave_children(
		&mut self,
		children: &mut Vec<Node<RustParts>>,
	) -> ParseResult<()> {
		let current_children = match self.out.nodes.last_mut() {
			Some(Node::Element(el)) => &mut el.children,
			Some(Node::Component(_, children)) => children,
			other => panic!("no node to add children to: {:?}", other),
		};
		current_children.append(children);
		self.position_visitor.leave_children(children)
	}
}

impl SweetRsxVisitor {

	fn map_children(&mut self, children: &Vec<Node<RustParts>>) -> Vec<Node<String>> {
		children
			.iter()
			.map(|child| {
				self.vis
			})
			.collect()
	}

	fn parse_attribute(
		&self,
		attribute: &mut Attribute<RustParts>,
	) -> ParseResult<Attribute<String>> {
		let attr = match attribute {
			Attribute::Key { key } => Attribute::Key {
				key: std::mem::take(key),
			},
			Attribute::KeyValue { key, value } => Attribute::KeyValue {
				key: std::mem::take(key),
				value: std::mem::take(value),
			},
			Attribute::BlockValue { key, value } => {
				let is_event = key.starts_with("on");
				let value = match (is_event, value) {
					(true, RustParts::Event(_)) => {
						format!(
							"_sweet.event({},event)",
							self.position_visitor.node_index
						)
					}
					(true, parts) => {
						return Err(ParseError::hydration(
							"RustParts::Event",
							parts,
						));
					}
					(false, RustParts::AttributeValue(val)) => {
						std::mem::take(val)
					}
					(false, val) => {
						return Err(ParseError::hydration(
							"RustParts::AttributeValue",
							val,
						))
					}
				};
				Attribute::BlockValue {
					key: std::mem::take(key),
					value,
				}
			}
			Attribute::Block(RustParts::AttributeBlock(block_str)) => {
				Attribute::Block(std::mem::take(block_str))
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


///	Encoding for TextBlock positions, we need the following:
/// - The child index of the text node
/// - The string index of the block
/// - The length of the TextBlock initial value
/// child_index - first-block-index , first-block-length , second-block-index , second-block-length . child_index2 etc
///
/// ## Example
/// ```html
/// <div>the 10th <bold>value</bold> was 9</div>
/// ```
/// Output:
/// 0-4,2.2-5,1
///
///
fn encode_text_block_positions(children: &Vec<Node<RustParts>>) -> String {
	let mut encoded = String::new();
	let mut child_index = 0;
	let mut text_index = 0;
	for child in children {
		match child {
			Node::Text(t) => {
				text_index += t.len();
			}
			Node::TextBlock(_) => {
				encoded.push_str(&format!("{},{},", child_index, text_index));
			}
			Node::Component(_, _) => {
				todo!("what if component returns text")
			}
			_ => {
				child_index += 1;
				text_index = 0;
				continue;
			}
		}
	}
	if encoded.len() > 0 {
		encoded.pop();
	}
	encoded
}
