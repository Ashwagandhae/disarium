use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use disarium::find_disarium;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("disarium 1 mil", |b| {
        b.iter(|| find_disarium(black_box(1_000_000)))
    });
    c.bench_function("disarium 10 mil", |b| {
        b.iter(|| find_disarium(black_box(100_000_000)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
