use crate::prelude::*;

pub trait Rsx {
	fn into_parts(self) -> RsxParts;
}

impl Rsx for RsxParts {
	fn into_parts(self) -> RsxParts { self }
}

impl Rsx for () {
	fn into_parts(self) -> RsxParts { RsxParts::default() }
}

pub trait Component {
	// type Props;
	// fn from_props(props: Self::Props) -> Self;
	fn render(self) -> impl Rsx;
}

impl<T: Component> Rsx for T {
	fn into_parts(self) -> RsxParts {
		let component = self.render();
		component.into_parts()
	}
}


pub trait Props {
	type Component;
	fn new() -> Self;
	fn build() -> Self::Component;
}
