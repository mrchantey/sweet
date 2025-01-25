mod hydrate;
mod rsx_node;
pub use hydrate::*;
pub use rsx_node::*;
pub use text_block_encoder::*;
mod text_block_encoder;

pub trait Rsx {
	fn into_rsx(self) -> RsxNode;
}

impl Rsx for RsxNode {
	fn into_rsx(self) -> RsxNode { self }
}
impl Rsx for () {
	fn into_rsx(self) -> RsxNode { RsxNode::default() }
}
// impl Rsx for &str {
// 	fn into_rsx(self) -> RsxNode { RsxNode::Text(self.to_string()) }
// }
// impl Rsx for String {
// 	fn into_rsx(self) -> RsxNode { RsxNode::Text(self) }
// }


pub trait IntoRsx<M> {
	fn into_rsx(self) -> RsxNode;
}

pub struct ToStringIntoRsx;
impl<T: ToString> IntoRsx<(T, ToStringIntoRsx)> for T {
	fn into_rsx(self) -> RsxNode { RsxNode::Text(self.to_string()) }
}
pub struct ToStringFuncIntoRsx;
impl<T: FnOnce() -> U, U: IntoRsx<M2>, M2> IntoRsx<(M2, ToStringFuncIntoRsx)>
	for T
{
	fn into_rsx(self) -> RsxNode { self().into_rsx() }
}


pub trait Component {
	fn render(self) -> impl Rsx;
}

impl<T: Component> Rsx for T {
	fn into_rsx(self) -> RsxNode { self.render().into_rsx() }
}
