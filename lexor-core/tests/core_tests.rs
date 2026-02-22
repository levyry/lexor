use lexor_core::de_bruijn::DeBruijn;
use lexor_core::kiselyov::{BulkResolver, kiselyov};

#[test]
fn test_pipeline() {
    // 1. Parse
    let input = "λx.λy.x";
    let lambda = lexor_core::parser::parse(input).unwrap();

    let deb: DeBruijn = From::from(lambda);

    let result = kiselyov(&deb, &BulkResolver::Logarithmic).1;

    assert_eq!(format!("{result}"), "C(BS(B(BB)(CB(SBI))))B(KI)BIKI");
}
