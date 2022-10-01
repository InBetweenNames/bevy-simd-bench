use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

use std::marker::PhantomData;

use bevy::{prelude::*, reflect::erased_serde::private::serde::de::IntoDeserializer};

struct X;
struct Y;
struct Z;

//Store a Vec4 for the "virtual entity" scheme
#[derive(Component)]
struct Position<Name>(Vec4, PhantomData<Name>);

impl<Name> From<Vec4> for Position<Name> {
    fn from(x: Vec4) -> Self {
        Self(x, PhantomData)
    }
}

#[derive(Bundle)]
struct PositionBundle {
    x: Position<X>,
    y: Position<Y>,
    z: Position<Z>,
}

#[derive(Component)]
struct Velocity<Name>(Vec4, PhantomData<Name>);

impl<Name> From<Vec4> for Velocity<Name> {
    fn from(x: Vec4) -> Self {
        Self(x, PhantomData)
    }
}

#[derive(Bundle)]
struct VelocityBundle {
    x: Velocity<X>,
    y: Velocity<Y>,
    z: Velocity<Z>,
}

impl VelocityBundle {
    fn new(xs: Vec4, ys: Vec4, zs: Vec4) -> Self {
        Self {
            x: xs.into(),
            y: ys.into(),
            z: zs.into(),
        }
    }
}

impl PositionBundle {
    fn new(xs: Vec4, ys: Vec4, zs: Vec4) -> Self {
        Self {
            x: xs.into(),
            y: ys.into(),
            z: zs.into(),
        }
    }
}

#[derive(Bundle)]
struct MovingBundle {
    position: PositionBundle,
    velocity: VelocityBundle,
}

impl MovingBundle {
    fn new(pxs: Vec4, pys: Vec4, pzs: Vec4, vxs: Vec4, vys: Vec4, vzs: Vec4) -> Self {
        Self {
            position: PositionBundle::new(pxs, pys, pzs),
            velocity: VelocityBundle::new(vxs, vys, vzs),
        }
    }
}

pub struct Benchmark<'w>(
    World,
    QueryState<(&'w Velocity<X>, &'w mut Position<X>)>,
    QueryState<(&'w Velocity<Y>, &'w mut Position<Y>)>,
    QueryState<(&'w Velocity<Z>, &'w mut Position<Z>)>,
    QueryState<(
        &'w Velocity<X>,
        &'w mut Position<X>,
        &'w Velocity<Y>,
        &'w mut Position<Y>,
        &'w Velocity<Z>,
        &'w mut Position<Z>,
    )>,
);

impl<'w> Benchmark<'w> {
    pub fn new(size: i32) -> Self {
        let size = size / 4; //4 "virtual entities" per identity

        let mut world = World::default();

        let mut rng = rand::thread_rng();

        world.spawn_batch((0..size).map(|_| {
            let pxs = Vec4::new(
                rng.gen_range(-16.0..=16.0),
                rng.gen_range(-16.0..=16.0),
                rng.gen_range(-16.0..=16.0),
                rng.gen_range(-16.0..=16.0),
            );
            let pys = Vec4::new(
                rng.gen_range(-16.0..=16.0),
                rng.gen_range(-16.0..=16.0),
                rng.gen_range(-16.0..=16.0),
                rng.gen_range(-16.0..=16.0),
            );
            let pzs = Vec4::new(
                rng.gen_range(-16.0..=16.0),
                rng.gen_range(-16.0..=16.0),
                rng.gen_range(-16.0..=16.0),
                rng.gen_range(-16.0..=16.0),
            );

            let vxs = Vec4::new(
                rng.gen_range(-16.0..=16.0),
                rng.gen_range(-16.0..=16.0),
                rng.gen_range(-16.0..=16.0),
                rng.gen_range(-16.0..=16.0),
            );
            let vys = Vec4::new(
                rng.gen_range(-16.0..=16.0),
                rng.gen_range(-16.0..=16.0),
                rng.gen_range(-16.0..=16.0),
                rng.gen_range(-16.0..=16.0),
            );
            let vzs = Vec4::new(
                rng.gen_range(-16.0..=16.0),
                rng.gen_range(-16.0..=16.0),
                rng.gen_range(-16.0..=16.0),
                rng.gen_range(-16.0..=16.0),
            );

            MovingBundle::new(pxs, pys, pzs, vxs, vys, vzs)
        }));

        let query_x = world.query::<(&Velocity<X>, &mut Position<X>)>();

        let query_y = world.query::<(&Velocity<Y>, &mut Position<Y>)>();

        let query_z = world.query::<(&Velocity<Z>, &mut Position<Z>)>();

        let query_full = world.query::<(
            &Velocity<X>,
            &mut Position<X>,
            &Velocity<Y>,
            &mut Position<Y>,
            &Velocity<Z>,
            &mut Position<Z>,
        )>();

        Self(world, query_x, query_y, query_z, query_full)
    }

    //TODO: show swizzling approach... also show how an incorrect access pattern will make things worse.

    pub fn run_optimal(&mut self, time: f32) {
        //Ensure sensible access patterns: if we merge the queries into one big query, then we'll incur more
        //cache misses as we'll be accessing x, y, and z in order, and they likely won't be near each other in memory.
        //Going in order is a more cache-friendly access pattern.

        self.1
            .for_each_mut(&mut self.0, |(velocity, mut position)| {
                position.0 += time * velocity.0;
            });
        self.2
            .for_each_mut(&mut self.0, |(velocity, mut position)| {
                position.0 += time * velocity.0;
            });
        self.3
            .for_each_mut(&mut self.0, |(velocity, mut position)| {
                position.0 += time * velocity.0;
            });
    }

    pub fn run_optimal_nochange(&mut self, time: f32) {
        //Ensure sensible access patterns: if we merge the queries into one big query, then we'll incur more
        //cache misses as we'll be accessing x, y, and z in order, and they likely won't be near each other in memory.
        //Going in order is a more cache-friendly access pattern.

        self.1
            .for_each_mut(&mut self.0, |(velocity, mut position)| {
                position.bypass_change_detection().0 += time * velocity.0;
            });
        self.2
            .for_each_mut(&mut self.0, |(velocity, mut position)| {
                position.bypass_change_detection().0 += time * velocity.0;
            });
        self.3
            .for_each_mut(&mut self.0, |(velocity, mut position)| {
                position.bypass_change_detection().0 += time * velocity.0;
            });
    }

    pub fn run_suboptimal(&mut self, time: f32) {
        self.4.for_each_mut(
            &mut self.0,
            |(
                velocity_x,
                mut position_x,
                velocity_y,
                mut position_y,
                velocity_z,
                mut position_z,
            )| {
                position_x.0 += time * velocity_x.0;
                position_y.0 += time * velocity_y.0;
                position_z.0 += time * velocity_z.0;
            },
        );
    }
}
