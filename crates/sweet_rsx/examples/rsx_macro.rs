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


// fn main() {
// 	let onclick = |_| {};

// 	let _foo = rsx! {
// 		<div onclick>
// 			<p>hello</p>
// 			<MyComponent value=7/>
// 		</div>
// 	};

// 	let str = _foo.to_info_string();
// 	println!("{}", str);
// }
fn main() {
	let foo = rsx! {
		<div>
			<p>hello</p>
		</div>
	};

	let str = foo.build_string();
	println!("{}", str);
}


fn boo() {
	let desc = "quick";
	let color = "brown";
	let action = "jumps over";

	let tree = rsx! {"The "{desc}" and "{color}<b> fox </b> {action}" the "<Adjective> and fat </Adjective>dog };
}
