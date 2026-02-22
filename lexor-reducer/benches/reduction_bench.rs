use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use lexor_reducer::{NF, ReductionStrat};

fn reduction_benchmark(c: &mut Criterion) {
    c.bench_function("church 2 20", |b| {
        b.iter(|| {
            NF::compute(black_box("(S(KS)K(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))(S(S(KS)K)I)KI"));
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = reduction_benchmark
}
criterion_main!(benches);
