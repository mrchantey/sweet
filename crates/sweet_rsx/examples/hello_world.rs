// #![cfg(target_arch = "wasm32")]
// use sweet_rsx::prelude::*;
// use web_sys::Event;


// pub struct HelloWorld;


// impl HydrateClient for HelloWorld {
// 	fn hydrate() -> ParseResult<Hydrated> {
// 		let handle_click = |e: Event| {
// 			e.target()
// 				.unwrap()
// 				.dyn_into::<web_sys::HtmlElement>()
// 				.unwrap()
// 				.set_inner_html("you did it!");
// 		};


// 		Ok(Hydrated {
// 			events: vec![Box::new(handle_click)],
// 		})
// 	}
// }



// fn main() { HelloWorld::hydrate(); }


fn main() {}
