use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use lexor_reducer::{NF, ReductionMachine};
use std::hint::black_box;

fn reduction_benchmark(c: &mut Criterion) {
    c.bench_function("church 2 20", |b| {
        b.iter(|| {
            let mut engine = ReductionMachine::from_string("(S(KS)K(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))(S(S(KS)K)I)KI");
            engine.compute::<NF>();
            black_box(engine)
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = reduction_benchmark
}
criterion_main!(benches);
