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
	fn render(self) -> impl Rsx;
}