use sweet::prelude::*;


fn main() -> ParseResult<()> {
	// Channel::send("test".into());
	// assert_eq!(Channel::receive(), "test");

	#[cfg(target_arch = "wasm32")]
	return loader();
	#[cfg(not(target_arch = "wasm32"))]
	return renderer();
}

#[cfg(target_arch = "wasm32")]
type Event = web_sys::Event;
#[cfg(not(target_arch = "wasm32"))]
type Event = ();

#[cfg(not(target_arch = "wasm32"))]
fn renderer() -> ParseResult<()> {
	let out = DefaultRsxRenderer::render(HelloWorld)?;
	forky::prelude::ReadFile::write(
		"target/hello_world/hello_world.html",
		&out,
	)?;

	Ok(())
}
pub struct HelloWorld;
impl Component for HelloWorld {
	fn render(self) -> impl Rsx {
		let mut count = 0;
		#[cfg(target_arch = "wasm32")]
		fn set_target_html(e: &Event, s: &str) {
			use wasm_bindgen::JsCast;
			e.target()
				.unwrap()
				.dyn_into::<web_sys::HtmlElement>()
				.unwrap()
				.set_inner_html(s);
		}
		let handle_click = move |_e: Event| {
			count += 1;
			let str = count.to_string();
			#[cfg(target_arch = "wasm32")]
			set_target_html(&e, &format!("you did it {str} times!"));
			Channel::send((0, str.clone()));
			Channel::send((1, str));
		};

		rsx! {
			<!DOCTYPE html>
			<html>
				<head>
					<title>{"Hello World"}</title>
					<button onclick=handle_click>{"Click me"}</button>
				</head>
				<body>
					<script>
					globalThis._sweet = {
						uncanny: [],
						event: (id, event) => {
							globalThis._sweet.uncanny.push([id, event])
						}
					}
				</script>
				<script type="module">
					import init from "./bindgen.js"
					setTimeout(async () => {
						await init()
						console.log("hydrated")
					}, 2000);
					</script>
					<h1>{"Hello World"}</h1>
					<p>{"This is a simple example of a sweet app"}</p>
				</body>
			</html>
		}
	}
}


#[cfg(target_arch = "wasm32")]
fn loader() -> ParseResult<()> {
	// #![cfg(target_arch = "wasm32")]

	SweetLoader::default().load(HelloWorld)?;
	Ok(())
}
use flume::Receiver;
use flume::Sender;
use once_cell::sync::Lazy;


static CHANNEL: Lazy<(Sender<(usize, String)>, Receiver<(usize, String)>)> =
	Lazy::new(|| flume::unbounded());

pub struct Channel;

impl Channel {
	pub fn send(msg: (usize, String)) {
		CHANNEL.0.send(msg).expect("Failed to send");
	}

	pub fn receive() -> (usize, String) {
		CHANNEL.1.recv().expect("Failed to receive")
	}
}
