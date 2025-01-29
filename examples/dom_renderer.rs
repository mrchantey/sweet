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
	use ::sweet::prelude::dom_mounter::DomMounter;

	console_error_panic_hook::set_once();

	let mut app = rsx! {<MyComponent initial=7/>};
	let doc = RsxToResumableHtml::default().map_node(&app);
	DomMounter::mount_doc(&doc);

	let mut hydrator = DomHydrator::default();
	hydrator.initialize();

	CurrentHydrator::set(hydrator);
	// CurrentHydrator::with(|h| h.initialize());
	app.register_effects();
}
