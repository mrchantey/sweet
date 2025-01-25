mod signal;
// use crate::rsx::RsxAttribute;
// use crate::rsx::RsxNode;
// use crate::rsx::RsxRust;
pub use signal::*;

pub struct SignalRsx;


// yes it looks right but i think these types will always be the same


// impl RsxRust for SignalRsx {
// 	type NodeBlock = NodeBlockEffect;
// 	type AttributeBlock = AttributeBlockEffect;
// 	type AttributeBlockValue = AttributeBlockValueEffect;

// 	fn block_to_string(block: &Self::NodeBlock) -> String {
// 		block.initial.build_string()
// 	}

// 	fn attribute_block_to_string(block: &Self::AttributeBlock) -> String {
// 		block
// 			.initial
// 			.iter()
// 			.map(|attr| attr.build_string())
// 			.collect()
// 	}

// 	fn attribute_block_value_to_string(
// 		block: &Self::AttributeBlockValue,
// 	) -> String {
// 		block.initial.clone()
// 	}
// }


// pub struct NodeBlockEffect {
// 	pub initial: RsxNode<SignalRsx>,
// 	pub register_effect: Box<dyn FnOnce()>,
// }

// impl NodeBlockEffect {
// 	pub fn new<T: Into<RsxNode<SignalRsx>>>(
// 		mut func: impl FnMut() -> T + 'static,
// 	) -> Self {
// 		Self {
// 			initial: func().into(),
// 			register_effect: Box::new(move || {
// 				effect(move || {
// 					let node: RsxNode<SignalRsx> = func().into();
// 					let node_str = node.build_string();
// 					println!("would update node: {:?}", node_str);
// 					todo!();
// 				})
// 			}),
// 		}
// 	}
// }


// pub struct AttributeBlockEffect {
// 	pub initial: Vec<RsxAttribute<SignalRsx>>,
// 	pub register_effect: Box<dyn FnOnce()>,
// }

// impl AttributeBlockEffect {
// 	pub fn new(
// 		mut func: impl FnMut() -> Vec<RsxAttribute<SignalRsx>> + 'static,
// 	) -> Self {
// 		Self {
// 			initial: func(),
// 			register_effect: Box::new(move || {
// 				effect(move || {
// 					let attributes = func();
// 					println!("would update attributes: {:?}", attributes.len());
// 					todo!();
// 				})
// 			}),
// 		}
// 	}
// }


// pub struct AttributeBlockValueEffect {
// 	pub initial: String,
// 	pub register_effect: Box<dyn FnOnce()>,
// }
// impl AttributeBlockValueEffect {
// 	pub fn new<T: Into<String>>(
// 		key: &str,
// 		mut func: impl FnMut() -> T + 'static,
// 	) -> Self {
// 		let key = key.to_string();
// 		Self {
// 			initial: func().into(),
// 			register_effect: Box::new(move || {
// 				effect(move || {
// 					let value: String = func().into();
// 					println!("would update attribute: {key}: {value}");
// 					todo!();
// 				})
// 			}),
// 		}
// 	}
// }
