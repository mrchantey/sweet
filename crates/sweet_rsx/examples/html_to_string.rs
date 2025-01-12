#![allow(non_camel_case_types)]
pub use sweet_rsx::html;
pub use sweet_rsx::html_ide;
// Using this parser, one can write docs and link html tags to them.
// if this macro would be independent, it would be nicer to have docs in
// separate crate.
pub mod docs {
	/// Element has open and close tags, content and attributes.
	pub fn element() {}
	/// Its a div!
	pub struct div;
	// pub fn div() {}
}


fn my_component() -> String {
	html! {<div>this is a component</div>}
}


fn main() {
	let component = my_component();

	// {val}
	let val = 2;
	let html = html! {
		<div
			id=val
			onclick={bang}
			// {age="foo"}
		>
			Hello,
		the value is {val}
		</div>
	};


	println!("html: {}", html);
}
