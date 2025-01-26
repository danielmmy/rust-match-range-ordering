use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use rust_match_range_ordering::{
    match_range_crescent_ordering, match_range_decrescent_ordering, run_many,
};

const TEST_SIZE: usize = 10_000_000_;

fn ordering_benchmark(c: &mut Criterion) {
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

criterion_group!(benches, ordering_benchmark);

criterion_main!(benches);
