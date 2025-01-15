#![allow(unused)]
use sweet::prelude::*;

fn main() {
	// This example

	// Server::default().run_once(());
	// Server::default().run_once(Counter { value: 7 }.render());
	// Server::default().run_once(Footer.render());
	// // TODO calling these.into_parts() and extending for children
	// Server::default().run_once(rsx! {<Counter value=3 />});
}

struct Counter {
	value: i32,
}

impl Component for Counter {
	fn render(self) -> impl Rsx {
		rsx! {
			<div> the value is {self.value} </div>
				<button onclick=|_| {}>Increment</button>
				// <button onclick={|_| {sweet::prelude::log!("hello world")}}>Increment</button>
			<Footer/>
		}
	}
}


struct Footer;

impl Component for Footer {
	fn render(self) -> impl Rsx {
		rsx! {
			<footer>
				<div>sweet as!</div>
			</footer>
		}
	}
}
