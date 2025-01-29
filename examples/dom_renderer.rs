use sweet_core as sweet;
use sweet_core::prelude::*;
use sweet_core::signals_rsx::signal;
use sweet_rsx_macros::rsx;

struct MyComponent {
	initial: u32,
}
#[allow(unused)]
impl Component for MyComponent {
	fn render(self) -> impl Rsx {
		let (value, set_value) = signal(self.initial);



		rsx! {
			<div>
				<div id="label">the value is {value}</div>
				// <button onclick={move |_| set_value(value() + 1)}>increment</button>
			</div>
		}
	}
}


fn main() { render(); }
#[cfg(not(target_arch = "wasm32"))]
fn render() {}

#[cfg(target_arch = "wasm32")]
fn render() {
	console_error_panic_hook::set_once();
	CurrentHydrator::set(DomHydrator::default());

	let mut app = rsx! {<MyComponent initial=7/>};

	let str = RsxToHtml::render_resumable(&app);

	// this would usually be directly served
	web_sys::window()
		.unwrap()
		.document()
		.unwrap()
		.body()
		.unwrap()
		.set_inner_html(&str);

	app.register_effects();
}
