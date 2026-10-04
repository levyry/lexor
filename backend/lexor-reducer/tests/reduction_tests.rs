#![allow(clippy::expect_used)]

use lexor_reducer::SkiReductionStrat;

#[test]
fn subexpressions() {
    let result =
        SkiReductionStrat::NormalForm.reduce("SS(IIIK)(KIS(SKSIKSKISKISKSI)KS(IKSKSISKIS)KIS)");

    assert_eq!(result, Ok("SS(KS)".to_owned()));
}

#[test]
fn church_expon() {
    // 2^3 = 8
    let result =
        SkiReductionStrat::NormalForm.reduce("(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))(S(S(KS)K)I)");

    assert_eq!(
        result,
        Ok("S(K(S(S(KS)K)I))(S(K(S(S(KS)K)I))(S(K(S(S(KS)K)I))(S(S(KS)K)I)))".to_owned())
    );
}

#[test]
fn normal_form_exhaustion() {
    let result = SkiReductionStrat::WeakHeadNormalForm.reduce("S(SKSKSSK)(SSKSKSK)");

    assert_eq!(result, Ok("S(SKSKSSK)(SSKSKSK)".to_owned()));
}

#[test]
fn ackermann_small() {
    let result =
        SkiReductionStrat::NormalForm.reduce("(S(S(KS)K)(S(S(KS)K)I))(S(S(KS)K)I)(S(S(KS)K)I)");

    assert_eq!(
        result,
        Ok("S(K(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)I)))(S(K(S(S(KS)K)I))(S(S(KS)K)I))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)I)))(S(K(S(S(KS)K)I))(S(S(KS)K)I)))".to_owned())
    );
}

#[test]
fn six_factorial() {
    let result = SkiReductionStrat::NormalForm.reduce(
        "(S(S(KS)K)(S(K(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I))))))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I))))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I))))))",
    );

    assert_eq!(
        result,
        Ok("S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))))))))(S(K(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))))))))(S(K(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))))))))(S(K(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))))))))(S(K(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))))))))))))".to_owned()));
}
#[test]
fn church_numeral_generation() {
    use lexor_core::combinator::Combinator;

    let zero: Combinator = Combinator::church(0);
    assert_eq!(zero.to_string(), "KI");

    let one: Combinator = Combinator::church(1);
    assert_eq!(one.to_string(), "I");

    let two: Combinator = Combinator::church(2);
    assert_eq!(two.to_string(), "S(S(KS)K)I");

    let three: Combinator = Combinator::church(3);
    assert_eq!(three.to_string(), "S(S(KS)K)(S(S(KS)K)I)");
}

#[test]
fn church_numeral_desugar() {
    use lexor_parser::parse_ski;

    let parsed = parse_ski("K 2 I").expect("parsing valid SKI with numeral");
    let desugared = parsed.desugar_church();
    assert_eq!(desugared.to_string(), "K(S(S(KS)K)I)I");

    let reduced = SkiReductionStrat::NormalForm.reduce(&desugared.to_string());
    assert_eq!(reduced, Ok("S(S(KS)K)I".to_owned()));
}

#[test]
fn step_by_step_arithmetic_reduction() {
    let mut steps = Vec::new();
    let res = SkiReductionStrat::WeakHeadNormalForm.reduce_with("(6 * 7) + (20 - 5)", |view| {
        steps.push(view.to_string());
    });
    assert_eq!(res, Ok("57".to_owned()));
    assert_eq!(
        steps,
        vec![
            "(6 * 7) + (20 - 5)",
            "42 + (20 - 5)",
            "42 + 15",
            "57",
        ]
    );

    let mut prefix_steps = Vec::new();
    let res2 = SkiReductionStrat::WeakHeadNormalForm.reduce_with("+ (* 6 7) (- 20 5)", |view| {
        prefix_steps.push(view.to_string());
    });
    assert_eq!(res2, Ok("57".to_owned()));
    assert_eq!(
        prefix_steps,
        vec![
            "(6 * 7) + (20 - 5)",
            "42 + (20 - 5)",
            "42 + 15",
            "57",
        ]
    );
}

