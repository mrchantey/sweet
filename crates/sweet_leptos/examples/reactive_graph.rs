use any_spawner::Executor;
use reactive_graph::effect::Effect;
use reactive_graph::owner::Owner;
use reactive_graph::prelude::*;
use reactive_graph::signal::RwSignal;
use std::sync::Arc;
use std::sync::RwLock;
use tokio::task;


// execution is super flaky, not sure whats going on
// maybe different tokio versions?
#[tokio::main]
async fn main() {
	_ = Executor::init_tokio();
	let owner = Owner::new();
	owner.set();

	task::LocalSet::new()
		.run_until(async {
			let a = RwSignal::new(-1);

			// simulate an arbitrary side effect
			let b = Arc::new(RwLock::new(String::new()));

			Effect::new({
				let b = b.clone();
				move || {
					let formatted = format!("Value is {}", a.get());
					*b.write().unwrap() = formatted;
				}
			});

			Executor::tick().await;
			assert_eq!(b.read().unwrap().as_str(), "Value is -1");

			println!("setting to 1");
			a.set(1);

			Executor::tick().await;
			assert_eq!(b.read().unwrap().as_str(), "Value is 1");
			println!("ok");
		})
		.await
}
