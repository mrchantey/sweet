use crate::prelude::*;


impl Rsx for RsxParts {
	fn into_parts(self) -> RsxParts { self }
}

impl Rsx for () {
	fn into_parts(self) -> RsxParts { RsxParts::default() }
}

pub trait Rsx {
	fn into_parts(self) -> RsxParts;
}


pub trait Component {
	fn render(self) -> impl Rsx;
}