use lexor_reducer::{ReductionMachine, ReductionMode};

#[test]
fn church_expon() {
    // 2^3 = 8
    let result =
        ReductionMachine::from_string("(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))(S(S(KS)K)I)").reduce();

    assert_eq!(
        result,
        "S(K(S(S(KS)K)I))(S(K(S(S(KS)K)I))(S(K(S(S(KS)K)I))(S(S(KS)K)I)))"
    );
}

#[test]
fn normal_form_exhaustion() {
    let result = ReductionMachine::from_string("S(SKSKSSK)(SSKSKSK)")
        .set_mode(ReductionMode::WeakHeadNormalForm)
        .reduce();

    assert_eq!(result, "S(SKSKSSK)(SSKSKSK)");
}

#[test]
fn ackermann_small() {
    let result =
        ReductionMachine::from_string("(S(S(KS)K)(S(S(KS)K)I))(S(S(KS)K)I)(S(S(KS)K)I)").reduce();

    assert_eq!(
        result,
        "S(K(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)I)))(S(K(S(S(KS)K)I))(S(S(KS)K)I))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)I)))(S(K(S(S(KS)K)I))(S(S(KS)K)I)))"
    );
}

#[test]
fn six_factorial() {
    let input = "(S(S(KS)K)(S(K(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I))))))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I))))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I))))))";

    let result = ReductionMachine::from_string(input).reduce();

    assert_eq!(
        result,
        "S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))))))))(S(K(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))))))))(S(K(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))))))))(S(K(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))))))))(S(K(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))))))))))))"
    );
}
