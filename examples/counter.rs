use sweet::prelude::*;

fn main() {
	// This example


	Server::default().run(());
	Server::default().run_once(Counter { value: 7 });
	Server::default().run_once(Footer);
	// TODO calling these.into_parts() and extending for children
	Server::default().run_once(rsx! {<Counter />});
}

struct Counter {
	value: i32,
}


impl Rsx for Counter {
	fn into_parts(self) -> RsxParts {
		rsx! {
			<div> the value is {self.value} </div>
				<button onclick=|_| {}>Increment</button>
				// <button onclick={|_| {sweet::prelude::log!("hello world")}}>Increment</button>
			<Footer/>
		}
	}
}


struct Footer;

impl Rsx for Footer {
	fn into_parts(self) -> RsxParts {
		rsx! {
			<footer>
				<div>sweet as!</div>
			</footer>
		}
	}
}
