use std::marker::PhantomData;

use bevy::prelude::*;

fn main() {
    App::new().add_startup_system(setup).run();
}

struct X;
struct Y;
struct Z;

#[derive(Component)]
struct Position<Name>(f32, PhantomData<Name>);

#[derive(Bundle)]
struct PositionBundle {
    x: Position<X>,
    y: Position<Y>,
    z: Position<Z>,
}

#[derive(Component)]
struct Velocity<Name>(f32, PhantomData<Name>);

#[derive(Bundle)]
struct VelocityBundle {
    x: Velocity<X>,
    y: Velocity<Y>,
    z: Velocity<Z>,
}

fn setup(mut commands: Commands) {}
