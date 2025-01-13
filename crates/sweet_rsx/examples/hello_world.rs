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
	fn hydrate(
		self,
		send: flume::Sender<(usize, String)>,
	) -> ParseResult<Hydrated> {
		let mut count = 0;


		let handle_click = move |e: Event| {
			count += 1;
			let str = count.to_string();
			set_target_html(&e, &format!("you did it {str} times!"));
			send.send((0, str.clone())).unwrap();
			send.send((1, str)).unwrap();
		};

		Ok(Hydrated {
			events: vec![Box::new(handle_click)],
			blocks: vec![
				HydratedBlock {
					node_id: 0,
					part_index: 1,
				},
				HydratedBlock {
					node_id: 0,
					part_index: 3,
				},
			],
		})
	}
}



fn main() -> ParseResult<()> {
	SweetLoader::default().load(HelloWorld)?;
	Ok(())
}
