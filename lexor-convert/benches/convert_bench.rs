use criterion::{Criterion, criterion_group, criterion_main};
use lexor_convert::*;
use std::hint::black_box;

fn convert_bench(c: &mut Criterion) {
    c.bench_function("lin_bulk", |b| {
        b.iter(|| {
            black_box(lin_bulk(Com::Cn(123)));
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = convert_bench
}
criterion_main!(benches);
