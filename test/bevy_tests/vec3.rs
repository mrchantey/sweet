use bevy::prelude::*;
use sweet::*;

#[derive(Debug, Clone, Copy, PartialEq, Deref, Component)]
struct Foo(pub Vec3);

#[sweet_test]
fn vec3() -> Result<()> {
    expect(Vec3::ZERO).to_be_close_to(Vec3::ZERO)?;
    expect(Vec3::ZERO).not().to_be_close_to(Vec3::ONE)?;
    expect(Foo(Vec3::ZERO).0).to_be_close_to(Vec3::ZERO)?;
    expect(Foo(Vec3::ZERO).0)
        .not()
        .to_be_close_to(Vec3::new(0.2, 0.2, 0.2))?;
    Ok(())
}
