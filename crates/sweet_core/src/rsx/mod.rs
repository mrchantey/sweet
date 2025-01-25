mod hydrate;
mod rsx_node;
mod rsx_rust;
pub use hydrate::*;
pub use rsx_node::*;
pub use rsx_rust::*;
pub use text_block_encoder::*;
mod text_block_encoder;

pub trait Rsx {
	fn into_rsx(self) -> RsxNode;
}



pub trait Component {
	fn render(self) -> impl Rsx;
}

impl<T: Component> Rsx for T {
	fn into_rsx(self) -> RsxNode { self.render().into_rsx() }
}


impl Rsx for RsxNode {
	fn into_rsx(self) -> RsxNode { self }
}

impl Rsx for () {
	fn into_rsx(self) -> RsxNode { RsxNode::default() }
}
impl Rsx for &str {
	fn into_rsx(self) -> RsxNode { RsxNode::Text(self.to_string()) }
}
impl Rsx for String {
	fn into_rsx(self) -> RsxNode { RsxNode::Text(self) }
}
