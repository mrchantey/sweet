use sweet::prelude::rsx;
use sweet::prelude::Rsx;
use sweet::prelude::RsxParts;

/// its my component
struct MyComponent {
	value: u32,
}
impl Rsx for MyComponent {
	fn into_parts(self) -> RsxParts {
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

	let str = _foo.html.load().unwrap().to_string_placeholder();
	println!("{}", str);
}
