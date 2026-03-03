use lexor_core::de_bruijn::DeBruijn;
use lexor_core::kiselyov::{BulkResolver, kiselyov};
use lexor_parser::lambda_parser::chumsky_parse as parse;

#[test]
fn test_pipeline() {
    // 1. Parse
    let input = "λx.λy.x";
    let lambda = parse(input).unwrap_or_else(|_| unreachable!());

    let deb: DeBruijn = lambda.into();

    let result = kiselyov(&deb, &BulkResolver::Logarithmic).1;

    assert_eq!(format!("{result}"), "C(BS(B(BB)(CB(SBI))))B(KI)BIKI");
}
