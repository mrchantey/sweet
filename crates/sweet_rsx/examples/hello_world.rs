#![cfg(target_arch = "wasm32")]
use sweet_rsx::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Event;


pub struct HelloWorld;

fn set_target_html(e: &Event, s: &str) {
	e.target()
		.unwrap()
		.dyn_into::<web_sys::HtmlElement>()
		.unwrap()
		.set_inner_html(s);
}

impl HydrateClient for HelloWorld {
	fn hydrate() -> ParseResult<Hydrated> {
		let mut count = 0;

		let handle_click = move |e: Event| {
			count += 1;
			set_target_html(&e, &format!("you did it {} times!", count));
		};

		Ok(Hydrated {
			events: vec![Box::new(handle_click)],
		})
	}
}



fn main() -> ParseResult<()> {
	let hydrated = HelloWorld::hydrate()?;
	SweetLoader::default().load(hydrated);
	Ok(())
}
