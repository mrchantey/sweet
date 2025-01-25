use sweet_core as sweet;
use sweet_core::string_rsx::*;
use sweet_rsx_macros::rsx;

/// its my component
struct MyComponent {
	value: u32,
}
impl Component for MyComponent {
	fn render(self) -> RsxNodes {
		rsx! {
			<div>{self.value}<slot/></div>
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
	assert_eq!(
		str,
		"<div><p>hello <div>38<div>some child</div></div></p></div>"
	);
	println!("{}", str);
}
