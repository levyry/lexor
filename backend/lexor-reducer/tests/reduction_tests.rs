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
fn native_arithmetic() {
    assert_eq!(
        SkiReductionStrat::NormalForm.reduce("+ 5 10"),
        Ok("15".to_owned())
    );
    assert_eq!(
        SkiReductionStrat::NormalForm.reduce("* 6 7"),
        Ok("42".to_owned())
    );
    assert_eq!(
        SkiReductionStrat::NormalForm.reduce("- 20 8"),
        Ok("12".to_owned())
    );
    assert_eq!(
        SkiReductionStrat::NormalForm.reduce("/ 30 5"),
        Ok("6".to_owned())
    );
    assert_eq!(
        SkiReductionStrat::NormalForm.reduce("^ 2 5"),
        Ok("32".to_owned())
    );
}

#[test]
fn nested_arithmetic_with_combinators() {
    assert_eq!(
        SkiReductionStrat::NormalForm.reduce("+ (+ 1 2) (+ 3 4)"),
        Ok("10".to_owned())
    );
    assert_eq!(
        SkiReductionStrat::NormalForm.reduce("K 42 (+ 1 2)"),
        Ok("42".to_owned())
    );
    assert_eq!(
        SkiReductionStrat::NormalForm.reduce("I (+ 10 20)"),
        Ok("30".to_owned())
    );
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
