use sweet::prelude::*;

pub fn main() {
	let html = Index::into_html().unwrap();

	println!("{}", html);
}
