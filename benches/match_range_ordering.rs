use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use rust_match_range_ordering::{
    match_crescent_ordering, match_decrescent_ordering, match_range_crescent_ordering,
    match_range_decrescent_ordering, run_many,
};

const TEST_SIZE: usize = 10_000_000_;

// compiles to lookup table, oredering should not matter.
fn match_ordering_benchmark(c: &mut Criterion) {
    let mut lower_data: Vec<usize> = vec![0; TEST_SIZE];
    let mut higher_data = vec![9; TEST_SIZE];
    for (idx, i) in lower_data.iter_mut().enumerate() {
        if (idx % 10) == 0 {
            *i = 9;
        }
    }

    for (idx, i) in higher_data.iter_mut().enumerate() {
        if (idx % 10) == 0 {
            *i = 0;
        }
    }

    c.bench_function("Lower values match with crescent ordering", |b| {
        b.iter(|| run_many(black_box(&lower_data), &match_crescent_ordering))
    });
    c.bench_function("Lower values match with decrescent ordering", |b| {
        b.iter(|| run_many(black_box(&lower_data), &match_decrescent_ordering))
    });

    c.bench_function("Higher values match with crescent ordering", |b| {
        b.iter(|| run_many(black_box(&higher_data), &match_crescent_ordering))
    });
    c.bench_function("Higher values match with decrescent ordering", |b| {
        b.iter(|| run_many(black_box(&higher_data), &match_decrescent_ordering))
    });
}

// compiles to sequence of if statements, order should matter
fn match_range_ordering_benchmark(c: &mut Criterion) {
    let mut lower_data: Vec<usize> = vec![0; TEST_SIZE];
    let mut higher_data = vec![95; TEST_SIZE];
    for (idx, i) in lower_data.iter_mut().enumerate() {
        if (idx % 10) == 0 {
            *i = 95;
        }
    }

    for (idx, i) in higher_data.iter_mut().enumerate() {
        if (idx % 10) == 0 {
            *i = 0;
        }
    }

    c.bench_function("Lower values match range with crescent ordering", |b| {
        b.iter(|| run_many(black_box(&lower_data), &match_range_crescent_ordering))
    });
    c.bench_function("Lower values match range with decrescent ordering", |b| {
        b.iter(|| run_many(black_box(&lower_data), &match_range_decrescent_ordering))
    });

    c.bench_function("Higher values match range with crescent ordering", |b| {
        b.iter(|| run_many(black_box(&higher_data), &match_range_crescent_ordering))
    });
    c.bench_function("Higher values match range with decrescent ordering", |b| {
        b.iter(|| run_many(black_box(&higher_data), &match_range_decrescent_ordering))
    });
}

criterion_group!(
    benches,
    match_ordering_benchmark,
    match_range_ordering_benchmark
);

criterion_main!(benches);
