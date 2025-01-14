use reactive_graph::signal::arc_signal;
use sweet::prelude::*;

pub struct Counter {
	initial_value: i32,
}


impl Rsx for Counter {
	fn into_parts(self) -> RsxParts {
		let (value, set_value) = arc_signal(self.initial_value);
		// rsx! {
		// 	<div>
		// 		<button onclick={|_|{set_value.update(|val|val -= 1)}}>-</button>
		// 		<span>The value is {value} for now</span>
		// 		<button onclick={|_|{set_value.update(|val|val += 1)}}>+</button>
		// 	</div>
		// }
	}
}
