use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use lexor_reducer::{GraphReductionEngine, ReductionMode};
use slotmap::SlotMap;
use std::hint::black_box;

fn reduction_benchmark(c: &mut Criterion) {
    let input = "((S (S (KS) K) I) ((S (S (KS) K) (S (S (KS) K) (S (S (KS) K) (S (S (KS) K) I)))) (S (S (KS) K) I)))";
    let parsed = lexor_reducer::parse(input).unwrap();

    c.bench_function("church 2 16 nf", |b| {
        b.iter(|| {
            let mut engine = GraphReductionEngine::<SlotMap<_, _>>::from_tree(parsed.clone())
                .set_mode(ReductionMode::NormalForm);
            engine.start();
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
