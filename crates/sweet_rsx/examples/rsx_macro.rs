use sweet_core as sweet;
use sweet_core::prelude::*;
use sweet_rsx_macros::rsx;

/// its my component
struct MyComponent {
	value: u32,
}
impl Component for MyComponent {
	fn render(self) -> impl Rsx {
		rsx! {
			<div>{self.value}</div>
		}
	}
}


fn main() {
	let onclick = |_| {};

	let _foo = rsx! {
		<div onclick>
			<p>hello</p>
			<MyComponent value=7/>
		</div>
	};

	let str = _foo.to_info_string();
	println!("{}", str);
}
