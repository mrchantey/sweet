use sweet::prelude::*;
fn main() {
	Server::default().run(sweet::noop! {
		{ sweet::prelude::RsxParts { rust : vec![], html :
		PathOrInline::Inline("<Counter value></Counter>".to_string()), css :
		PathOrInline::Inline("".to_string()), } }
	});
}
struct Counter {
	value: i32,
}
impl IntoRsx for Counter {
	fn into_rsx(mut self) -> impl Rsx {
		sweet::noop! {
			{ sweet::prelude::RsxParts { rust : vec![sweet::prelude::RsxRust::InnerText({
			self.value } .to_string()), RsxRust::Event(Box::new({ | _ | { self.value +=
			1; } })),], html :
			PathOrInline::Inline("<div rsx-id=\"0\"> the value is  §</div><button rsx-id=\"1\" onclick=\"§\">Increment</button><Footer></Footer>"
			.to_string()), css : PathOrInline::Inline("".to_string()), } }
		}
	}
}
struct Footer;
impl IntoRsx for Footer {
	fn into_rsx(self) -> impl Rsx {
		sweet::noop! {
			{ sweet::prelude::RsxParts { rust : vec![], html :
			PathOrInline::Inline("<footer><div>sweet as!</div></footer>".to_string()),
			css : PathOrInline::Inline("".to_string()), } }
		}
	}
}
