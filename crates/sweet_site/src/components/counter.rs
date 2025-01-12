use reactive_graph::signal::arc_signal;
use sweet_rsx::prelude::*;

pub struct Counter {
	initial_value: i32,
}


impl IntoRsx for Counter {
	fn into_rsx(self) {
		let (value, set_value) = arc_signal(self.initial_value);
		rsx! {
			<div>
				<button onclick={|_|{set_value.update(|val|val -= 1)}}>-</button>
				<span>The value is {value} for now</span>
				<button onclick={|_|{set_value.update(|val|val += 1)}}>+</button>
			</div>
		}
	}
}
