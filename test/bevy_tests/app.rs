use bevy::prelude::*;
use sweet::*;

#[derive(Debug, PartialEq, Component, Resource)]
pub struct Health(pub u32);

sweet! {

    test "world"{
        let mut world = World::new();
        expect(&world).not().to_contain_resource::<Health>()?;
        world.insert_resource(Health(5));
        expect(&world).to_contain_resource::<Health>()?;
    }

    test "app" {
        let mut app = App::new();
        let entity = app.world_mut().spawn_empty().id();

        expect(app.world()).not().to_have_component::<Health>(entity)?;
        app.world_mut().entity_mut(entity).insert(Health(7));
        expect(app.world()).to_have_component::<Health>(entity)?;
        expect(app.world()).component(entity)?.to_be(&Health(7))?;

        expect(app.world()).not().to_contain_resource::<Health>()?;
        app.world_mut().insert_resource(Health(5));
        expect(app.world()).to_contain_resource::<Health>()?;
        expect(app.world()).resource()?.to_be(&Health(5))?;

        expect(app.world()).not().to_contain_non_send_resource::<Health>()?;
        app.world_mut().insert_non_send_resource(Health(5));
        expect(app.world()).to_contain_non_send_resource::<Health>()?;
        expect(app.world()).non_send_resource()?.to_be(&Health(5))?;
    }
}
