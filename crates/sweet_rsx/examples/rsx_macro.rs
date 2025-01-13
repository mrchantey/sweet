use sweet_rsx::rsx;

pub fn main() {
	let planet = "mars";
	let id = 32;
	// let val = rsx! {
	// 	<button
	// 		id={id}
	// 		onclick={|_|{alert(format!("hello {}!", planet))}}>
	// 		>
	// 			Click Me
	// 	</button>
	// };
	// println!("{}", val);
}

fn alert(msg: String) {
	println!("{}", msg);
}
