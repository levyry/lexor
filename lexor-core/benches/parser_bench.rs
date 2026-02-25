use criterion::{Criterion, criterion_group, criterion_main};
use lexor_parser::lambda_parser::chumsky_parse as parse;
use std::hint::black_box;

fn parse_bench(c: &mut Criterion) {
    let input = "(\\a.\\b.\\c.b(a b c))((\\a.\\b.\\c.a b(a b(a b(a b(a b(a b c))))))(\\a.\\b.a(a(a(a(a(a(a b))))))))(\\a.\\b.a(a(a(a(a(a(a(a(a(a(a(a b))))))))))))";

    c.bench_function("hand written", |b| {
        b.iter(|| black_box(parse(input).unwrap_or_else(|_| unreachable!())));
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = parse_bench
}
criterion_main!(benches);
