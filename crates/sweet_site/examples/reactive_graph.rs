fn main() {
	use reactive_graph::computed::ArcMemo;
	use reactive_graph::effect::Effect;
	use reactive_graph::prelude::Read;
	use reactive_graph::prelude::Set;
	use reactive_graph::signal::ArcRwSignal;

	let count = ArcRwSignal::new(1);
	let double_count = ArcMemo::new({
		let count = count.clone();
		move |_| *count.read() * 2
	});
	let double_count2 = double_count.clone();

	// the effect will run once initially
	Effect::new(move |_| {
		println!("double_count = {}", *double_count.read());
	});

	// updating `count` will propagate changes to the dependencies,
	// causing the effect to run again

	count.set(2);
	assert_eq!(*double_count2.read(), 4);
	// success but effect didnt run
}