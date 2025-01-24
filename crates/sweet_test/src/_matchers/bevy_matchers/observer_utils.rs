use bevy::prelude::*;
use sweet::prelude::*;

type Func<T> = MockFunc<T, T, fn(T) -> T>;

pub fn observe_triggers<E: Event + Clone>(world: &mut World) -> Func<E> {
	let func: Func<E> = mock_func(|a| a);
	let func2 = func.clone();
	world.add_observer(move |on_result: Trigger<E>| {
		func2(on_result.event().clone());
	});
	func
}

pub fn observe_trigger_names<E: Event>(world: &mut World) -> Func<String> {
	let func: Func<String> = mock_func(|a| a);
	let func2 = func.clone();
	world.add_observer(move |on_result: Trigger<E>, query: Query<&Name>| {
		if let Ok(name) = query.get(on_result.entity()) {
			func2(name.to_string());
		}
	});
	func
}
