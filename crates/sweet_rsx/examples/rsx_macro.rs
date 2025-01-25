use sweet_core as sweet;
use sweet_core::string_rsx::*;
use sweet_rsx_macros::rsx;

/// its my component
struct MyComponent {
	value: u32,
	children: Children,
}
impl Component for MyComponent {
	fn render(self) -> RsxNodes {
		rsx! {
			<div>{self.value}</div>
		}
	}
}

fn main() {
	let foo = rsx! {
		<div>
			<p>hello <MyComponent value=38><div>some child</div></MyComponent></p>
		</div>
	};

	let str = foo.build_string();
	println!("{}", str);
}
