use crate::prelude::*;
use strum_macros::AsRefStr;
use strum_macros::EnumDiscriminants;




pub type RegisterEffect = Box<dyn FnOnce(&RsxContext)>;



/// This struct represents one of the core concepts of sweet rsx!
///
// #[derive(Debug, Clone, PartialEq)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(AsRefStr, EnumDiscriminants)]
pub enum RsxNode {
	/// A transparent node that simply contains children
	Fragment(Vec<RsxNode>),
	/// a rust block that returns text
	Block {
		initial: Box<RsxNode>,
		register_effect: RegisterEffect,
	},
	Doctype,
	Comment(String),
	/// may have been Text or RawText
	Text(String),
	Element(RsxElement),
}

impl Default for RsxNode {
	fn default() -> Self { Self::Fragment(Vec::new()) }
}

pub struct RenderOptions {
	/// add attributes required for resumability
	resumable: bool,
	/// For every html node visited, this will increment by 1.
	/// If resumable, the id will be attached to the html.
	cx: RsxContext,
	html_constants: HtmlConstants,
}

impl Default for RenderOptions {
	fn default() -> Self {
		Self {
			resumable: false,
			html_constants: Default::default(),
			cx: Default::default(),
		}
	}
}


impl RenderOptions {
	pub fn resumable() -> Self {
		Self {
			resumable: true,
			..Default::default()
		}
	}
}


impl RsxNode {
	/// Render with default [IntoHtmlOptions]
	pub fn render(&self) -> String {
		self.render_with_options(&mut Default::default())
	}
	/// Render with default [IntoHtmlOptions]
	pub fn render_with_options(&self, options: &mut RenderOptions) -> String {
		self.into_html(options).render()
	}

	pub fn into_discriminant(&self) -> RsxNodeDiscriminants {
		match self {
			RsxNode::Doctype => RsxNodeDiscriminants::Doctype,
			RsxNode::Comment(_) => RsxNodeDiscriminants::Comment,
			RsxNode::Text(_) => RsxNodeDiscriminants::Text,
			RsxNode::Block { .. } => RsxNodeDiscriminants::Block,
			RsxNode::Element(_) => RsxNodeDiscriminants::Element,
			RsxNode::Fragment(_) => RsxNodeDiscriminants::Fragment,
		}
	}

	pub fn into_html(&self, options: &mut RenderOptions) -> Vec<HtmlNode> {
		let desc = self.into_discriminant();
		options.cx.before_visit_next(&desc);
		let val = match self {
			RsxNode::Doctype => {
				vec![HtmlNode::Doctype]
			}
			RsxNode::Comment(s) => {
				vec![HtmlNode::Comment(s.clone())]
			}
			RsxNode::Text(s) => {
				vec![HtmlNode::Text(s.clone())]
			}
			RsxNode::Block { initial, .. } => initial.into_html(options),
			RsxNode::Element(e) => {
				vec![HtmlNode::Element(e.into_html(options))]
			}
			RsxNode::Fragment(nodes) => nodes
				.iter()
				.map(|n| n.into_html(options))
				.flatten()
				.collect(),
		};
		options.cx.after_visit_next(&desc);
		val
	}

	/// A method used by macros to insert nodes into a slot
	/// # Panics
	/// If the slot is not found
	pub fn with_slots(mut self, name: &str, nodes: Vec<RsxNode>) -> Self {
		match self.try_insert_slots(name, nodes) {
			Some(_) => {
				panic!("slot not found: {name}");
			}
			None => return self,
		}
	}

	// try to insert nodes into a slot, returning any nodes that were not inserted
	// If the slot is not a direct child, recursively search children
	pub fn try_insert_slots(
		&mut self,
		name: &str,
		mut nodes: Vec<Self>,
	) -> Option<Vec<Self>> {
		match self {
			RsxNode::Fragment(fragment) => {
				for node in fragment.iter_mut() {
					match node.try_insert_slots(name, nodes) {
						Some(returned_nodes) => nodes = returned_nodes,
						None => return None,
					}
				}
				Some(nodes)
			}
			RsxNode::Element(element) => {
				if element.tag == "slot" {
					let slot_name = element
						.attributes
						.iter()
						.find_map(|a| match a {
							RsxAttribute::KeyValue { key, value } => {
								if key == "name" {
									Some(value.as_str())
								} else {
									None
								}
							}
							// even block values are not allowed because we need slot names at macro time
							_ => None,
						})
						// unnamed slots are called 'default'
						.unwrap_or("default");
					if slot_name == name {
						element.children.extend(nodes);
						return None;
					}
				}
				// if we didnt find the slot, recursively search children
				for child in &mut element.children {
					match child.try_insert_slots(name, nodes) {
						Some(returned_nodes) => nodes = returned_nodes,
						None => return None,
					}
				}
				Some(nodes)
			}
			_ => Some(nodes),
		}
	}

	/// takes all the register_effect functions
	pub fn register_effects(&mut self) -> RsxContext {
		let mut cx = RsxContext::default();
		self.register_effects_recursive(&mut cx);
		cx
	}

	fn register_effects_recursive(&mut self, cx: &mut RsxContext) {
		let desc = self.into_discriminant();
		cx.before_visit_next(&desc);

		fn call_effect(cx: &RsxContext, register_effect: &mut RegisterEffect) {
			let func = std::mem::replace(register_effect, Box::new(|_| {}));
			func(cx);
		}

		match self {
			RsxNode::Block {
				register_effect, ..
			} => {
				call_effect(cx, register_effect);
			}
			RsxNode::Element(e) => {
				for a in &mut e.attributes {
					match a {
						RsxAttribute::Block {
							register_effect, ..
						} => call_effect(cx, register_effect),
						RsxAttribute::BlockValue {
							register_effect, ..
						} => call_effect(cx, register_effect),
						_ => {}
					}
				}
				cx.before_element_children();
				for c in &mut e.children {
					c.register_effects_recursive(cx);
				}
				cx.after_element_children();
			}
			RsxNode::Fragment(nodes) => {
				for n in nodes {
					n.register_effects_recursive(cx);
				}
			}
			RsxNode::Doctype => {}
			RsxNode::Comment(_) => {}
			RsxNode::Text(_) => {}
		}
		cx.after_visit_next(&desc);
	}
}

pub struct RsxElement {
	/// ie `div, span, input`
	pub tag: String,
	/// ie `class="my-class"`
	pub attributes: Vec<RsxAttribute>,
	/// ie `<div>childtext<childel/>{childblock}</div>`
	pub children: Vec<RsxNode>,
	/// ie `<input/>`
	pub self_closing: bool,
}


impl RsxElement {
	pub fn new(tag: String, self_closing: bool) -> Self {
		Self {
			tag,
			self_closing,
			attributes: Vec::new(),
			children: Vec::new(),
		}
	}

	pub fn into_html(&self, options: &mut RenderOptions) -> HtmlElementNode {
		let mut attributes = self
			.attributes
			.iter()
			.map(|a| a.into_html(options))
			.flatten()
			.collect::<Vec<_>>();

		if options.resumable {
			if self.contains_rust() {
				attributes.push(HtmlAttribute {
					key: options.html_constants.id_attribute_key.to_string(),
					value: Some(options.cx.rsx_id().to_string()),
				});
			}
			if self.contains_blocks() {
				attributes.push(HtmlAttribute {
					key: options.html_constants.block_attribute_key.to_string(),
					value: Some(TextBlockEncoder::encode(&self)),
				});
			}
		}

		HtmlElementNode {
			tag: self.tag.clone(),
			self_closing: self.self_closing,
			attributes,
			children: {
				options.cx.before_element_children();
				let children = self
					.children
					.iter()
					.map(|c| c.into_html(options))
					.flatten()
					.collect();
				options.cx.after_element_children();
				children
			},
		}
	}

	/// non-recursive check for blocks in children
	pub fn contains_blocks(&self) -> bool {
		self.children
			.iter()
			.any(|c| matches!(c, RsxNode::Block { .. }))
	}

	/// Whether any children or attributes are blocks,
	/// used to determine whether the node requires an id
	pub fn contains_rust(&self) -> bool {
		self.contains_blocks()
			|| self.attributes.iter().any(|a| {
				matches!(
					a,
					RsxAttribute::Block { .. }
						| RsxAttribute::BlockValue { .. }
				)
			})
	}
}

// #[derive(Debug, Clone, PartialEq)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RsxAttribute {
	Key {
		key: String,
	},
	KeyValue {
		key: String,
		value: String,
	},
	BlockValue {
		key: String,
		initial: String,
		register_effect: RegisterEffect,
	},
	// kind of like a fragment, but for attributes
	Block {
		initial: Vec<RsxAttribute>,
		register_effect: RegisterEffect,
	},
}

impl RsxAttribute {
	pub fn render(&self) -> String {
		self.into_html(&mut Default::default()).render()
	}


	pub fn into_html(&self, options: &RenderOptions) -> Vec<HtmlAttribute> {
		match self {
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
				.map(|a| a.into_html(options))
				.flatten()
				.collect(),
		}
	}
}
