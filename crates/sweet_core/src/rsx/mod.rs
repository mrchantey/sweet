mod hydrate;
mod rsx_rust;
mod rsx_tree;
pub use hydrate::*;
pub use rsx_rust::*;
pub use rsx_tree::*;
pub use rust_parts::*;
pub use text_block_encoder::*;
#[cfg(feature = "tokens")]
mod rsx_tree_quote;
mod rust_parts;
mod text_block_encoder;

pub trait Rsx {
	type Node: RsxRust;
	fn into_rsx_tree(self) -> RsxTree<Self::Node>;
}

// impl<R: RsxRust> Rsx for RsxTree<R> {
// 	fn into_rsx_tree(self) -> RsxTree<impl RsxRust> { self }
// }

// impl Rsx for () {
// 	fn into_rsx_tree(self) -> RsxTree<impl RsxRust> { RsxTree::<()>::default() }
// }
// impl Rsx for &str {
// 	fn into_rsx_tree(self) -> RsxTree<impl RsxRust> {
// 		RsxTree::<StringRsx>::new(vec![RsxNode::Text(self.to_string())])
// 	}
// }
// impl Rsx for String {
// 	fn into_rsx_tree(self) -> RsxTree<impl RsxRust> {
// 		RsxTree::<StringRsx>::new(vec![RsxNode::Text(self)])
// 	}
// }
