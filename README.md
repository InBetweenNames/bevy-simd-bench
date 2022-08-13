# Bevy SIMD Benchmarks
---

This crate consists of a number of benchmarks intended for testing out the effects of data layouts and batching strategies on Bevy
systems and queries.

To run:
~~~
cargo bench
~~~

The results will land in your `target/criterion/report` directory when finished.

These benchmarks were created in support of Bevy issue [#1990](https://github.com/bevyengine/bevy/issues/1990)

# What is tested?

The effect of AoS, SoA, and AoSoA layouts on Bevy queries, and different batching methods.

The benchmarks are divided into 6 categories: 

* naive: AoS layout without alignment (e.g., Vec3)
* naive_aligned: AoS layout with alignment (e.g., Vec3 with an extra component to get 16 bytes alignment)
* naive_batched_swizzle: Operates on batches of 4 Vec3s, converts their layouts to SoA, does the processing, and swizzles the results back
* aosoa_sse4: uses an AoSoA layout with 4 lanes
* simd_batch_sse4: uses SoA layout.  the "optimal" and "suboptimal" benchmarks show differences between different iteration patterns.

# How?

For SoA and AoSoA layouts, I implemented the tests using upstream Bevy by having each "entity" manage 4 "virtual entities".
This provides the same data layout as if Bevy itself supported AoSoA and SoA by default.  Then, each system operates on batches of 4
"virtual entities" using SIMD to simulate the results.  The total number of entities across all benchmarks (within the same size category)
are the same.

The actual tests involve a loop that updates an Entity's position using a time and velocity.  Inputs are randomized to prevent the compiler from
optimizing them out, and provide a more realistic distribution.

# Results

![Results](https://user-images.githubusercontent.com/7820684/184507249-cdbec2dd-be84-447b-841d-9bd2113be838.png)

So far, AoSoA seems to be winning on my system, followed by the SoA layout, followed by the AoS layouts.

# Do these results generalize?

I expect, based on the numbers provided, that they do, and a significant speedup is on the table for systems using dense components
and linear access patterns.

# Current limitations

* This crate relies on the plain mathematical types provided by Bevy
* No fancy packed_simd support, etc
* These benchmarks do not test the effect of `-march=native`, fast math optimizations, etc.
* Only `f32`s tested.  No integral types.