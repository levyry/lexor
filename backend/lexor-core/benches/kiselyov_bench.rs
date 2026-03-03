use criterion::{Criterion, criterion_group, criterion_main};
use lexor_core::{de_bruijn::DeBruijn, kiselyov::kiselyov};
use lexor_parser::lambda_parser::chumsky_parse as parse;
use std::hint::black_box;

fn kiselyov_bench(c: &mut Criterion) {
    let input = "λx.λy.x";
    let lambda = parse(input).unwrap_or_else(|_| unreachable!());

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
