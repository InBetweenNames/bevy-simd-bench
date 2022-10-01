use rand::Rng;

use std::marker::PhantomData;

use bevy::prelude::*;

use bevy::ptr::AlignedArrayTrait;

struct X;
struct Y;
struct Z;

#[derive(Component)]
#[repr(transparent)]
struct Position<Name>(f32, PhantomData<Name>);

impl<Name> From<f32> for Position<Name> {
    fn from(x: f32) -> Self {
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
#[repr(transparent)]
struct Velocity<Name>(f32, PhantomData<Name>);

impl<Name> From<f32> for Velocity<Name> {
    fn from(x: f32) -> Self {
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
    fn new(xs: f32, ys: f32, zs: f32) -> Self {
        Self {
            x: xs.into(),
            y: ys.into(),
            z: zs.into(),
        }
    }
}

impl PositionBundle {
    fn new(xs: f32, ys: f32, zs: f32) -> Self {
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
    fn new(pxs: f32, pys: f32, pzs: f32, vxs: f32, vys: f32, vzs: f32) -> Self {
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

fn as_vec4(vec: &bevy::ptr::AlignedArray16<f32, 4>) -> &Vec4 {
    unsafe {
        // Safety: vec is aligned properly
        &*(vec.as_ptr() as *const Vec4)
    }
}

fn as_vec4_mut(vec: &mut bevy::ptr::AlignedArray16<f32, 4>) -> &mut Vec4 {
    unsafe {
        // Safety: vec is aligned properly
        &mut *(vec.as_ptr() as *mut Vec4)
    }
}

fn as_vec4_position<Name>(vec: &bevy::ptr::AlignedArray16<Position<Name>, 4>) -> &Vec4 {
    unsafe {
        // Safety: vec is aligned properly
        &*(vec.as_ptr() as *const Vec4)
    }
}

fn as_vec4_position_mut<Name>(vec: &mut bevy::ptr::AlignedArray16<Position<Name>, 4>) -> &mut Vec4 {
    unsafe {
        // Safety: vec is aligned properly, Position<Name> is repr(transprent), bevy::ptr::AlignedArray16<Position<Name>, 4> is repr(transparent)
        &mut *(vec.as_ptr() as *mut Vec4)
    }
}

fn as_vec4_velocity<Name>(vec: &bevy::ptr::AlignedArray16<Velocity<Name>, 4>) -> &Vec4 {
    unsafe {
        // Safety: vec is aligned properly
        &*(vec.as_ptr() as *const Vec4)
    }
}

fn as_vec4_velocity_mut<Name>(vec: &mut bevy::ptr::AlignedArray16<Velocity<Name>, 4>) -> &mut Vec4 {
    unsafe {
        // Safety: vec is aligned properly
        &mut *(vec.as_ptr() as *mut Vec4)
    }
}

impl<'w> Benchmark<'w> {
    pub fn new(size: i32) -> Self {
        let mut world = World::default();

        let mut rng = rand::thread_rng();

        world.spawn_batch((0..size).map(|_| {
            let pxs = rng.gen_range(-16.0..=16.0);
            let pys = rng.gen_range(-16.0..=16.0);
            let pzs = rng.gen_range(-16.0..=16.0);

            let vxs = rng.gen_range(-16.0..=16.0);
            let vys = rng.gen_range(-16.0..=16.0);
            let vzs = rng.gen_range(-16.0..=16.0);

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

    pub fn run_nochange(&mut self, time: f32) {
        //Ensure sensible access patterns: if we merge the queries into one big query, then we'll incur more
        //cache misses as we'll be accessing x, y, and z in order, and they likely won't be near each other in memory.
        //Going in order is a more cache-friendly access pattern.

        self.1.for_each_mut_batched::<4, 16, _, _>(
            &mut self.0,
            |(velocity, mut position)| {
                position.0 += time * velocity.0;
            },
            |(velocity, mut position)| {
                let p = position.bypass_change_detection();

                let v = *as_vec4_velocity(velocity);
                let p = as_vec4_position_mut(p);

                *p += time * v;
            },
        );
        self.2.for_each_mut_batched::<4, 16, _, _>(
            &mut self.0,
            |(velocity, mut position)| {
                position.0 += time * velocity.0;
            },
            |(velocity, mut position)| {
                let p = position.bypass_change_detection();

                let v = *as_vec4_velocity(velocity);
                let p = as_vec4_position_mut(&mut *p);

                *p += time * v;
            },
        );
        self.3.for_each_mut_batched::<4, 16, _, _>(
            &mut self.0,
            |(velocity, mut position)| {
                position.0 += time * velocity.0;
            },
            |(velocity, mut position)| {
                let p = position.bypass_change_detection();

                let v = *as_vec4_velocity(velocity);
                let p = as_vec4_position_mut(&mut *p);

                *p += time * v;
            },
        );
    }

    pub fn run_change(&mut self, time: f32) {
        //Ensure sensible access patterns: if we merge the queries into one big query, then we'll incur more
        //cache misses as we'll be accessing x, y, and z in order, and they likely won't be near each other in memory.
        //Going in order is a more cache-friendly access pattern.

        self.1.for_each_mut_batched::<4, 16, _, _>(
            &mut self.0,
            |(velocity, mut position)| {
                position.0 += time * velocity.0;
            },
            |(velocity, mut position)| {
                let v = *as_vec4_velocity(velocity);
                let p = as_vec4_position_mut(&mut *position);

                *p += time * v;
            },
        );
        self.2.for_each_mut_batched::<4, 16, _, _>(
            &mut self.0,
            |(velocity, mut position)| {
                position.0 += time * velocity.0;
            },
            |(velocity, mut position)| {
                let v = *as_vec4_velocity(velocity);
                let p = as_vec4_position_mut(&mut *position);

                *p += time * v;
            },
        );
        self.3.for_each_mut_batched::<4, 16, _, _>(
            &mut self.0,
            |(velocity, mut position)| {
                position.0 += time * velocity.0;
            },
            |(velocity, mut position)| {
                let v = *as_vec4_velocity(velocity);
                let p = as_vec4_position_mut(&mut *position);

                *p += time * v;
            },
        );
    }
}
