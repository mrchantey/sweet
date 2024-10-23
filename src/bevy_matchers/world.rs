use crate::matchers::*;
use anyhow::Result;
use bevy::prelude::*;
use extend::ext;
use std::ops::Deref;

#[ext(name=MatcherMutExtWorld)]
/// Matcher extensions for `bevy::World`
pub impl<'a, W> Matcher<W>
where
    W: 'a + Into<&'a mut World>,
{
    fn num_components<T: Component>(self) -> Matcher<usize> {
        let world = self.value.into();
        let mut arr = world.query::<&T>();
        let received = arr.iter(world).count();
        Matcher::new(received)
    }
}

#[ext(name=MatcherExtWorld)]
/// Matcher extensions for `bevy::World`
pub impl<'a, W> Matcher<W>
where
    W: 'a + Deref<Target = World>,
{
    fn to_have_entity(&self, entity: Entity) -> Result<()> {
        let value = self.value.deref();
        let received = value.get_entity(entity);
        self.assert_option_with_received_negatable(received.ok())
    }

    fn to_have_component<T: Component>(&self, entity: Entity) -> Result<()> {
        let received = self.value.deref().get::<T>(entity);
        self.assert_option_with_received_negatable(received)
    }

    fn component<T: Component>(&self, entity: Entity) -> Result<Matcher<&T>> {
        let received = self.value.deref().get::<T>(entity);
        self.assert_option_with_received(received)
            .map(|c| Matcher::new(c))
    }

    fn to_contain_resource<T: Resource>(&self) -> Result<()> {
        let received = self.value.deref().get_resource::<T>();
        self.assert_option_with_received_negatable(received)
    }

    fn resource<T: Resource>(&self) -> Result<Matcher<&T>> {
        let received = self.value.deref().get_resource::<T>();
        self.assert_option_with_received(received)
            .map(|c| Matcher::new(c))
    }

    fn to_contain_non_send_resource<T: 'static>(&self) -> Result<()> {
        let received = self.value.deref().get_non_send_resource::<T>();
        self.assert_option_with_received_negatable(received)
    }

    fn non_send_resource<T: 'static>(&self) -> Result<Matcher<&T>> {
        let received = self.value.deref().get_non_send_resource::<T>();
        self.assert_option_with_received(received)
            .map(|c| Matcher::new(c))
    }

    //breaks backtracing
    // fn component_to_be<T>(
    // 	&self,
    // 	entity: impl SweetInto<Entity>,
    // 	other: &T,
    // ) -> Result<()>
    // where
    // 	T: Component + PartialEq + std::fmt::Debug,
    // {
    // 	self.component::<T>(entity)?.to_be(other)
    // }
}
