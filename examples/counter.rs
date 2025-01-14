use sweet::prelude::*;

fn main() { Server::default().run(rsx! {<Counter value=21/>}); }

struct Counter {
	value: i32,
}


impl IntoRsx for Counter {
	fn into_rsx(mut self) -> impl Rsx {
		rsx! {
			<div> the value is {self.value} </div>
			<button onclick={|_| {self.value += 1;}}>Increment</button>
			<Footer/>
		}
	}
}


struct Footer;

impl IntoRsx for Footer {
	fn into_rsx(self) -> impl Rsx {
		rsx! {
			<footer>
				<div>sweet as!</div>
			</footer>
		}
	}
}
