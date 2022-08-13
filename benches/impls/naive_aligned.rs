use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;

use std::marker::PhantomData;

use bevy::math::Vec3A;
use bevy::prelude::*;

#[derive(Component, Copy, Clone)]
struct Position(Vec3A);

#[derive(Component, Copy, Clone)]
struct Velocity(Vec3A);

pub struct Benchmark<'w>(World, QueryState<(&'w Velocity, &'w mut Position)>);

impl<'w> Benchmark<'w> {
    pub fn new(size: i32) -> Self {
        let mut world = World::new();

        let mut rng = rand::thread_rng();

        world.spawn_batch((0..size).map(|_| {
            let x1 = rng.gen_range(-16.0..=16.0);
            let x2 = rng.gen_range(-16.0..=16.0);
            let x3 = rng.gen_range(-16.0..=16.0);

            let v1 = rng.gen_range(-16.0..=16.0);
            let v2 = rng.gen_range(-16.0..=16.0);
            let v3 = rng.gen_range(-16.0..=16.0);

            (
                Position(Vec3A::new(x1, x2, x3)),
                Velocity(Vec3A::new(v1, v2, v3)),
            )
        }));

        let query = world.query::<(&Velocity, &mut Position)>();
        Self(world, query)
    }

    pub fn run(&mut self, time: f32) {
        self.1
            .for_each_mut(&mut self.0, |(velocity, mut position)| {
                position.0 += time * velocity.0;
            });
    }
}
