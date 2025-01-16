use crate::prelude::*;

pub trait Rsx {
	fn into_rsx_tree(self) -> RsxTree<RustParts>;
}

impl Rsx for RsxTree<RustParts> {
	fn into_rsx_tree(self) -> RsxTree<RustParts> { self }
}

impl Rsx for () {
	fn into_rsx_tree(self) -> RsxTree<RustParts> { Default::default() }
}
impl Rsx for &str {
	fn into_rsx_tree(self) -> RsxTree<RustParts> {
		RsxTree::new(vec![Node::Text(self.to_string())])
	}
}
impl Rsx for String {
	fn into_rsx_tree(self) -> RsxTree<RustParts> {
		RsxTree::new(vec![Node::Text(self)])
	}
}


pub trait Component {
	// type Props;
	// fn from_props(props: Self::Props) -> Self;
	fn render(self) -> impl Rsx;
}

impl<T: Component> Rsx for T {
	fn into_rsx_tree(self) -> RsxTree<RustParts> {
		let component = self.render();
		component.into_rsx_tree()
	}
}


pub trait Props {
	type Component;
	fn new() -> Self;
	fn build() -> Self::Component;
}
