use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use disarium::find_disarium;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("disarium 10mil", |b| {
        b.iter(|| find_disarium(black_box(100)).collect::<Vec<_>>())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
