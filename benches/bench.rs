#![feature(portable_simd)]
#![feature(slice_as_chunks)]

use bevy::prelude::Vec3;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use impls::vec3::Explosion;
use rand::prelude::*;

mod impls;

fn generate_explosion(rng: &mut ThreadRng) -> Explosion {
    Explosion {
        center: Vec3::new(
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
        ),
        radius_squared: rng.gen_range(0.0..=1.0),
    }
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("simd_benchmarks");
    group.warm_up_time(std::time::Duration::from_secs(1));
    group.measurement_time(std::time::Duration::from_secs(9));

    //TODO: does it make sense to let Criterion specify the world size??

    for exp in 14..16 {
        let size = 2_i32.pow(exp) - 1; //Ensure scalar path gets run too

        group.throughput(criterion::Throughput::Elements(size as u64));

        /*group.bench_with_input(BenchmarkId::new("vec3", size), &size, |b, &size| {
            let mut bench = impls::vec3::Benchmark::new(size);
            b.iter(move || bench.run(rand::thread_rng().gen_range(0.0..=1.0)));

            /*b.iter_batched_ref(
                || impls::naive::Benchmark::new(size),
                move |b| b.run(rand::thread_rng().gen::<f32>()),
                BatchSize::SmallInput,
            )*/
        });*/

        group.bench_with_input(
            BenchmarkId::new("vec3_nochangedetect", size),
            &size,
            |b, &size| {
                let mut bench = impls::vec3::Benchmark::new(size);
                b.iter(move || bench.run_nochange(generate_explosion(&mut rand::thread_rng())));

                /*b.iter_batched_ref(
                    || impls::naive::Benchmark::new(size),
                    move |b| b.run(rand::thread_rng().gen::<f32>()),
                    BatchSize::SmallInput,
                )*/
            },
        );

        /*group.bench_with_input(
            BenchmarkId::new("naive_aligned", size),
            &size,
            |b, &size| {
                let mut bench = impls::naive_aligned::Benchmark::new(size);
                b.iter(move || bench.run(rand::thread_rng().gen_range(0.0..=1.0)));
            },
        );*/

        group.bench_with_input(
            BenchmarkId::new("soa_batch_4_nochangedetect", size),
            &size,
            |b, &size| {
                let mut bench = impls::soa::Benchmark::new(size);
                b.iter(move || bench.run_nochange_4(generate_explosion(&mut rand::thread_rng())));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("soa_batch_8_nochangedetect", size),
            &size,
            |b, &size| {
                let mut bench = impls::soa::Benchmark::new(size);
                b.iter(move || bench.run_nochange_8(generate_explosion(&mut rand::thread_rng())));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("soa_nochangedetect", size),
            &size,
            |b, &size| {
                let mut bench = impls::soa::Benchmark::new(size);
                b.iter(move || bench.run_nochange(generate_explosion(&mut rand::thread_rng())));
            },
        );

        //Performs same as above
        /*
        group.bench_with_input(
            BenchmarkId::new("soa_batch_16_nochangedetect", size),
            &size,
            |b, &size| {
                let mut bench = impls::soa::Benchmark::new(size);
                b.iter(move || bench.run_nochange_16(rand::thread_rng().gen_range(0.0..=1.0)));
            },
        );
        */

        /*group.bench_with_input(BenchmarkId::new("soa_batch_4", size), &size, |b, &size| {
            let mut bench = impls::soa::Benchmark::new(size);
            b.iter(move || bench.run_4(rand::thread_rng().gen_range(0.0..=1.0)));
        });*/

        /*
        group.bench_with_input(BenchmarkId::new("aosoa_sse4", size), &size, |b, &size| {
            let mut bench = impls::aosoa_sse4::Benchmark::new(size); // TODO: rename f32x4
            b.iter(move || bench.run(rand::thread_rng().gen_range(0.0..=1.0)));
        });
        */

        /*group.bench_with_input(
            BenchmarkId::new("vec3_batch_4_swizzle", size),
            &size,
            |b, &size| {
                let mut bench = impls::vec3::Benchmark::new(size); // TODO: rename f32x4
                b.iter(move || bench.run_swizzle::<4>(rand::thread_rng().gen_range(0.0..=1.0)));
            },
        );*/

        group.bench_with_input(
            BenchmarkId::new("vec3_batch_4_swizzle_nochangedetect", size),
            &size,
            |b, &size| {
                let mut bench = impls::vec3::Benchmark::new(size); // TODO: rename f32x4
                b.iter(move || {
                    //bench.run_swizzle_nochange::<4>(rand::thread_rng().gen_range(0.0..=1.0))
                    bench.run_swizzle_nochange_4(generate_explosion(&mut rand::thread_rng()))
                });
            },
        );

        /*group.bench_with_input(
            BenchmarkId::new("vec3_batch_8_swizzle", size),
            &size,
            |b, &size| {
                let mut bench = impls::vec3::Benchmark::new(size); // TODO: rename f32x4
                b.iter(move || bench.run_swizzle::<8>(rand::thread_rng().gen_range(0.0..=1.0)));
            },
        );*/

        group.bench_with_input(
            BenchmarkId::new("vec3_batch_8_swizzle_nochangedetect", size),
            &size,
            |b, &size| {
                let mut bench = impls::vec3::Benchmark::new(size); // TODO: rename f32x4
                b.iter(move || {
                    //bench.run_swizzle_nochange::<8>(rand::thread_rng().gen_range(0.0..=1.0))
                    bench.run_swizzle_nochange_8(generate_explosion(&mut rand::thread_rng()))
                });
            },
        );

        /*group.bench_with_input(
            BenchmarkId::new("vec3_batch_8_nochangedetect", size),
            &size,
            |b, &size| {
                let mut bench = impls::vec3::Benchmark::new(size); // TODO: rename f32x4
                b.iter(move || {
                    //bench.run_swizzle_nochange::<8>(rand::thread_rng().gen_range(0.0..=1.0))
                    bench.run_batch_nochange::<8>(rand::thread_rng().gen_range(0.0..=1.0))
                });
            },
        );*/

        /*group.bench_with_input(
            BenchmarkId::new("vec3_batch_16_nochangedetect", size),
            &size,
            |b, &size| {
                let mut bench = impls::vec3::Benchmark::new(size); // TODO: rename f32x4
                b.iter(move || {
                    //bench.run_swizzle_nochange::<8>(rand::thread_rng().gen_range(0.0..=1.0))
                    bench.run_batch_nochange::<16>(rand::thread_rng().gen_range(0.0..=1.0))
                });
            },
        );*/

        /*group.bench_with_input(
            BenchmarkId::new("vec3_batch_16_swizzle", size),
            &size,
            |b, &size| {
                let mut bench = impls::vec3::Benchmark::new(size); // TODO: rename f32x4
                b.iter(move || bench.run_swizzle::<16>(rand::thread_rng().gen_range(0.0..=1.0)));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("vec3_batch_16_swizzle_nochangedetect", size),
            &size,
            |b, &size| {
                let mut bench = impls::vec3::Benchmark::new(size); // TODO: rename f32x4
                b.iter(move || {
                    bench.run_swizzle_nochange::<16>(rand::thread_rng().gen_range(0.0..=1.0))
                });
            },
        );*/

        /*
        group.bench_with_input(
            BenchmarkId::new("simd_batch_4_soa_simulated", size),
            &size,
            |b, &size| {
                let mut bench = impls::simd_batch_sse4::Benchmark::new(size);
                b.iter(move || bench.run_optimal(rand::thread_rng().gen_range(0.0..=1.0)));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("simd_batch_4_soa_simulated_nochangedetect", size),
            &size,
            |b, &size| {
                let mut bench = impls::simd_batch_sse4::Benchmark::new(size);
                b.iter(move || bench.run_optimal_nochange(rand::thread_rng().gen_range(0.0..=1.0)));
            },
        );
        */
    }

    //TODO: ensure LTO, O3, -ffast-math, etc...
    //NOTE: worth using ints to isolate effects of that?

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
