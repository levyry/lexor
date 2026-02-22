use lexor_reducer::{NF, ReductionStrat, WHNF};

#[test]
fn church_expon() {
    // 2^3 = 8
    let result = NF::reduce("(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))(S(S(KS)K)I)");

    assert_eq!(
        result,
        "S(K(S(S(KS)K)I))(S(K(S(S(KS)K)I))(S(K(S(S(KS)K)I))(S(S(KS)K)I)))"
    );
}

#[test]
fn normal_form_exhaustion() {
    let result = WHNF::reduce("S(SKSKSSK)(SSKSKSK)");

    assert_eq!(result, "S(SKSKSSK)(SSKSKSK)");
}

#[test]
fn ackermann_small() {
    let result = NF::reduce("(S(S(KS)K)(S(S(KS)K)I))(S(S(KS)K)I)(S(S(KS)K)I)");

    assert_eq!(
        result,
        "S(K(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)I)))(S(K(S(S(KS)K)I))(S(S(KS)K)I))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)I)))(S(K(S(S(KS)K)I))(S(S(KS)K)I)))"
    );
}

#[test]
fn six_factorial() {
    let result = NF::reduce(
        "(S(S(KS)K)(S(K(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I))))))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I))))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I))))))",
    );

    assert_eq!(
        result,
        "S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))))))))(S(K(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))))))))(S(K(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))))))))(S(K(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))))))))(S(K(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))))))))))))"
    );
}
