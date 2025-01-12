use reactive_graph::signal::arc_signal;
use sweet_rsx::rsx;

pub struct Props {
	initial_value: i32,
}

pub fn Counter(props: Props) {
	let (value, set_value) = arc_signal(props.initial_value);

	rsx! {
		<div>
			<button onclick={|_|{set_value.update(|val|val -= 1)}}>-</button>
			<span>The value is {value} for now</span>
			<button onclick={|_|{set_value.update(|val|val += 1)}}>+</button>
		</div>
	}
}