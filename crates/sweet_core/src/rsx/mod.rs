mod hydrate;
mod rsx_rust;
mod rsx_node;
pub use hydrate::*;
pub use rsx_rust::*;
pub use rsx_node::*;
pub use text_block_encoder::*;
mod text_block_encoder;

pub trait Rsx {
	fn into_rsx(self) -> RsxNode;
}



pub trait Component {
	fn render(self) -> RsxNode;
}

impl<T: Component> Rsx for T {
	fn into_rsx(self) -> RsxNode { self.render() }
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
