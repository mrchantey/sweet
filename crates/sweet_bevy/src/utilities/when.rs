use bevy::ecs::component::ComponentId;
use bevy::ecs::storage::ResourceData;
use bevy::ecs::system::ReadOnlySystemParam;
use bevy::ecs::system::SystemMeta;
use bevy::ecs::system::SystemParam;
use bevy::ecs::system::SystemParamValidationError;
use bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell;
use bevy::prelude::*;




/// An alternative to types like [`Res`] that should `skip` instead of panic when they dont exist.
pub struct When<'a, T> {
	pub value: &'a T,
}

impl<'a, T> When<'a, T> {
	pub fn new(value: &'a T) -> Self { Self { value } }
}

// SAFETY: Res only reads a single World resource
unsafe impl<'a, T: Resource> ReadOnlySystemParam for When<'a, T> {}

unsafe impl<'a, T: Resource> SystemParam for When<'a, T> {
	type State = ComponentId;
	type Item<'w, 's> = When<'w, T>;

	fn init_state(
		world: &mut World,
		system_meta: &mut bevy::ecs::system::SystemMeta,
	) -> Self::State {
		Res::<'a, T>::init_state(world, system_meta)
	}

	unsafe fn get_param<'w, 's>(
		state: &'s mut Self::State,
		system_meta: &bevy::ecs::system::SystemMeta,
		world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'w>,
		change_tick: bevy::ecs::component::Tick,
	) -> Self::Item<'w, 's> {
		unsafe {
			let value: Res<'w, T> =
				Res::get_param(state, system_meta, world, change_tick);
			When::new(value.into_inner())
		}
	}
	unsafe fn validate_param(
		&component_id: &Self::State,
		_system_meta: &SystemMeta,
		world: UnsafeWorldCell,
	) -> Result<(), SystemParamValidationError> {
		// SAFETY: Read-only access to resource metadata.
		if unsafe { world.storages() }
			.resources
			.get(component_id)
			.is_some_and(ResourceData::is_present)
		{
			Ok(())
		} else {
			Err(SystemParamValidationError::skipped::<Self>(
				"Resource does not exist",
			))
		}
	}
}

/// An alternative to types like [`ResMut`] that should `skip` instead of panic when they dont exist.
pub struct WhenMut<'a, T> {
	pub value: &'a mut T,
}

impl<'a, T> WhenMut<'a, T> {
	pub fn new(value: &'a mut T) -> Self { Self { value } }
}

// SAFETY: Res only reads a single World resource
unsafe impl<'a, T: Resource> ReadOnlySystemParam for WhenMut<'a, T> {}

unsafe impl<'a, T: Resource> SystemParam for WhenMut<'a, T> {
	type State = ComponentId;
	type Item<'w, 's> = WhenMut<'w, T>;

	fn init_state(
		world: &mut World,
		system_meta: &mut bevy::ecs::system::SystemMeta,
	) -> Self::State {
		ResMut::<'a, T>::init_state(world, system_meta)
	}

	unsafe fn get_param<'w, 's>(
		state: &'s mut Self::State,
		system_meta: &bevy::ecs::system::SystemMeta,
		world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'w>,
		change_tick: bevy::ecs::component::Tick,
	) -> Self::Item<'w, 's> {
		unsafe {
			let value: ResMut<'w, T> =
				ResMut::get_param(state, system_meta, world, change_tick);
			WhenMut::new(value.into_inner())
		}
	}
	unsafe fn validate_param(
		&component_id: &Self::State,
		_system_meta: &SystemMeta,
		world: UnsafeWorldCell,
	) -> Result<(), SystemParamValidationError> {
		// SAFETY: Read-only access to resource metadata.
		if unsafe { world.storages() }
			.resources
			.get(component_id)
			.is_some_and(ResourceData::is_present)
		{
			Ok(())
		} else {
			Err(SystemParamValidationError::skipped::<Self>(
				"Resource does not exist",
			))
		}
	}
}


#[cfg(test)]
mod test {
	use crate::prelude::*;
	use bevy::prelude::*;

	#[derive(Default, Resource)]
	struct Foo;

	#[test]
	#[should_panic]
	fn default() { App::new().add_systems(Update, |_res: Res<Foo>| {}).run(); }

	#[test]
	#[should_panic]
	fn panics() {
		App::new()
			.init_resource::<Foo>()
			.add_systems(Update, |_res: When<Foo>| {
				panic!("this will be reached")
			})
			.run();
	}
	#[test]
	fn doesnt_panic() {
		App::new()
			.add_systems(Update, |_res: When<Foo>| {
				panic!("this wont be reached")
			})
			.run();
	}
}
