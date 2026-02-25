use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use lexor_parser::ski_parser::chumsky_parse as parse;
use lexor_reducer::core::engine::Engine;
use lexor_reducer::{EngineView, NF, ReductionStrat};

#[allow(clippy::unwrap_used)]
fn church(c: &mut Criterion) {
    let root = parse("(S(KS)K(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))(S(S(KS)K)I)KI").unwrap();

    c.bench_function("church 2 20", |b| {
        b.iter(|| {
            let engine = black_box(Engine::from_tree(black_box(root.clone())));
            NF::perform(&mut black_box(engine), &mut None::<fn(EngineView)>);
        });
    });

    c.bench_function("church 2 20 cb", |b| {
        b.iter(|| {
            let engine = black_box(Engine::from_tree(black_box(root.clone())));
            NF::perform(
                &mut black_box(engine),
                &mut Some(|view: EngineView<'_>| {
                    let _s = view.stack_depth();
                }),
            );
        });
    });
}

criterion_group! {
    name = church_bench;
    config = Criterion::default().sample_size(100);
    targets = church
}
criterion_main!(church_bench);
