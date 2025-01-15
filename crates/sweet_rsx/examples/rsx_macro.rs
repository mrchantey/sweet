use sweet::prelude::rsx;
use sweet::prelude::Rsx;
use sweet::prelude::RsxParts;


struct Component {
	value: u32,
}
impl Rsx for Component {
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
			<Component value=7/>
		</div>
	};

	let str = _foo.html.load().unwrap().to_string_placeholder();
	println!("{}", str);
}
