use criterion::{Criterion, criterion_group, criterion_main};
use lexor_core::{de_bruijn::DeBruijn, kiselyov::kiselyov};
use std::hint::black_box;

fn kiselyov_bench(c: &mut Criterion) {
    let input = "λx.λy.x";
    let lambda = lexor_core::parser::parse(input).unwrap();

    let deb: DeBruijn = lambda.into();

    c.bench_function("hand written", |b| {
        b.iter(|| {
            let (_, ski) = kiselyov(&deb, &lexor_core::kiselyov::BulkResolver::Logarithmic);
            black_box(ski);
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = kiselyov_bench
}
criterion_main!(benches);
