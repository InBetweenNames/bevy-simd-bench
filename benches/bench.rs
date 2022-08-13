use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use rand::prelude::*;

mod impls;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("simd_benchmarks");
    group.warm_up_time(std::time::Duration::from_secs(1));
    group.measurement_time(std::time::Duration::from_secs(9));

    //TODO: does it make sense to let Criterion specify the world size??

    for exp in 14..21 {
        let size = 2_i32.pow(exp);

        //Each iteration processes (effectively) 3 f32s, one iteration processes size * 3 * f32 bytes
        group.throughput(criterion::Throughput::Elements(size as u64));

        group.bench_with_input(BenchmarkId::new("naive", size), &size, |b, &size| {
            let mut bench = impls::naive::Benchmark::new(size);
            b.iter(move || bench.run(rand::thread_rng().gen_range(0.0..=1.0)));

            /*b.iter_batched_ref(
                || impls::naive::Benchmark::new(size),
                move |b| b.run(rand::thread_rng().gen::<f32>()),
                BatchSize::SmallInput,
            )*/
        });

        group.bench_with_input(
            BenchmarkId::new("naive_aligned", size),
            &size,
            |b, &size| {
                let mut bench = impls::naive_aligned::Benchmark::new(size);
                b.iter(move || bench.run(rand::thread_rng().gen_range(0.0..=1.0)));
            },
        );

        //Each iteration processes 4 "virtual entities" by mapping one virtual entity to each SIMD lane.
        //Therefore the total number of entities are the same
        group.throughput(criterion::Throughput::Elements(size as u64));

        group.bench_with_input(BenchmarkId::new("aosoa_sse4", size), &size, |b, &size| {
            let mut bench = impls::aosoa_sse4::Benchmark::new(size); // TODO: rename f32x4
            b.iter(move || bench.run(rand::thread_rng().gen_range(0.0..=1.0)));
        });

        group.bench_with_input(
            BenchmarkId::new("naive_batched_swizzle", size),
            &size,
            |b, &size| {
                let mut bench = impls::naive_batched_swizzle::Benchmark::new(size); // TODO: rename f32x4
                b.iter(move || bench.run(rand::thread_rng().gen_range(0.0..=1.0)));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("simd_batch_sse4_soa_optimal", size),
            &size,
            |b, &size| {
                let mut bench = impls::simd_batch_sse4::Benchmark::new(size);
                b.iter(move || bench.run_optimal(rand::thread_rng().gen_range(0.0..=1.0)));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("simd_batch_sse4_soa_suboptimal", size),
            &size,
            |b, &size| {
                let mut bench = impls::simd_batch_sse4::Benchmark::new(size);
                b.iter(move || bench.run_suboptimal(rand::thread_rng().gen_range(0.0..=1.0)));
            },
        );
    }

    //TODO: ensure LTO, O3, -ffast-math, etc...
    //NOTE: worth using ints to isolate effects of that?

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
