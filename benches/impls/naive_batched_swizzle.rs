use bevy::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;

use std::marker::PhantomData;

use bevy::prelude::*;

//"Batched" naive Vec3 requiring swizzling

#[derive(Component, Copy, Clone)]
struct Position([Vec3; 4]);

#[derive(Component, Copy, Clone)]
struct Velocity([Vec3; 4]);

pub struct Benchmark<'w>(World, QueryState<(&'w Velocity, &'w mut Position)>);

impl<'w> Benchmark<'w> {
    pub fn new(size: i32) -> Self {
        let size = size / 4; //4 virtual entities per entity

        let mut world = World::new();

        let mut rng = rand::thread_rng();

        world.spawn_batch((0..size).map(|_| {
            let ps = [0, 1, 2, 3].map(|x| {
                let x1 = rng.gen_range(-16.0..=16.0);
                let x2 = rng.gen_range(-16.0..=16.0);
                let x3 = rng.gen_range(-16.0..=16.0);

                Vec3::new(x1, x2, x3)
            });

            let vs = [0, 1, 2, 3].map(|x| {
                let x1 = rng.gen_range(-16.0..=16.0);
                let x2 = rng.gen_range(-16.0..=16.0);
                let x3 = rng.gen_range(-16.0..=16.0);

                Vec3::new(x1, x2, x3)
            });
            (Position(ps), Velocity(vs))
        }));

        let query = world.query::<(&Velocity, &mut Position)>();
        Self(world, query)
    }

    pub fn run(&mut self, time: f32) {
        self.1
            .for_each_mut(&mut self.0, |(velocity, mut position)| {
                //Swizzle (hope it optimizes lol)

                //NOTE: look at assembly to ensure benchmark is fair

                let pxs = Vec4::new(
                    position.0[0].x,
                    position.0[1].x,
                    position.0[2].x,
                    position.0[3].x,
                );
                let pys = Vec4::new(
                    position.0[0].y,
                    position.0[1].y,
                    position.0[2].y,
                    position.0[3].y,
                );
                let pzs = Vec4::new(
                    position.0[0].z,
                    position.0[1].z,
                    position.0[2].z,
                    position.0[3].z,
                );

                let vxs = Vec4::new(
                    velocity.0[0].x,
                    velocity.0[1].x,
                    velocity.0[2].x,
                    velocity.0[3].x,
                );
                let vys = Vec4::new(
                    velocity.0[0].y,
                    velocity.0[1].y,
                    velocity.0[2].y,
                    velocity.0[3].y,
                );
                let vzs = Vec4::new(
                    velocity.0[0].z,
                    velocity.0[1].z,
                    velocity.0[2].z,
                    velocity.0[3].z,
                );

                //Do the vectorized math

                let nxs = pxs + time * vxs;
                let nys = pys + time * vys;
                let nzs = pzs + time * vzs;

                //Now re-arrange it back

                for i in 0..4 {
                    position.0[i].x = nxs[i];
                    position.0[i].y = nys[i];
                    position.0[i].z = nzs[i];
                }
            });

        //NOTE: should use integer math to avoid -ffast-math complications?
    }
}
