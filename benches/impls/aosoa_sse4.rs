use bevy::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;

use std::marker::PhantomData;

use bevy::prelude::*;

//Each AoSoAVec3 is 4-wide
//FIXME: use nalgebra to use their SIMD types?  or simba?
#[derive(Copy, Clone)]
struct AoSoAVec3 {
    v: [Vec4; 3], //I hope this gets laid out correctly
}

impl AoSoAVec3 {
    fn new(x: Vec4, y: Vec4, z: Vec4) -> Self {
        AoSoAVec3 { v: [x, y, z] }
    }
}

#[derive(Component, Copy, Clone)]
struct Position(AoSoAVec3);

#[derive(Component, Copy, Clone)]
struct Velocity(AoSoAVec3);

pub struct Benchmark<'w>(World, QueryState<(&'w Velocity, &'w mut Position)>);

impl<'w> Benchmark<'w> {
    pub fn new(size: i32) -> Self {
        let size = size / 4; //4 virtual entities per entity

        let mut world = World::new();

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

            (
                Position(AoSoAVec3::new(pxs, pys, pzs)),
                Velocity(AoSoAVec3::new(vxs, vys, vzs)),
            )
        }));

        let query = world.query::<(&Velocity, &mut Position)>();
        Self(world, query)
    }

    pub fn run(&mut self, time: f32) {
        self.1
            .for_each_mut(&mut self.0, |(velocity, mut position)| {
                position.0.v[0] += time * velocity.0.v[0];
                position.0.v[1] += time * velocity.0.v[1];
                position.0.v[2] += time * velocity.0.v[2];
            });
    }
}
