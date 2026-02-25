use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use lexor_parser::ski_parser::chumsky_parse as parse;
use lexor_reducer::{EngineView, NF, ReductionStrat, core::engine::Engine};

fn reduction_benchmark(c: &mut Criterion) {
    let root = parse("(S(KS)K(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))(S(S(KS)K)I)KI").unwrap();

    c.bench_function("church 2 20", |b| {
        b.iter(|| {
            let mut engine = black_box(Engine::from_tree(black_box(root.clone())));
            NF::perform(&mut engine, &mut None::<fn(EngineView)>);
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = reduction_benchmark
}
criterion_main!(benches);
