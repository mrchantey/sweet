use crate::prelude::*;

pub trait Rsx {
	fn into_parts(self) -> RsxParts;
}
impl Rsx for RsxParts {
	fn into_parts(self) -> RsxParts { self }
}

impl Rsx for () {
	fn into_parts(self) -> RsxParts { Default::default() }
}
// impl Rsx<()> for () {
// 	fn into_rsx(self) -> RsxParts { Default::default() }
// }

pub trait IntoRsx {
	fn into_rsx(self) -> impl Rsx;
}
