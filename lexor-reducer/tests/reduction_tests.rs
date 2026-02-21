use lexor_reducer::graphred::GenericGraphReductionMachine;
use lexor_reducer::graphred::ReductionMode;
use slotmap::SlotMap;

#[test]
fn church_expon() {
    // 2^3 = 8
    let input = "(S (S (KS) K) (S (S (KS) K) (S (S (KS) K) I))) (S (S (KS) K) I)";
    let parsed = lexor_reducer::parse(input).unwrap();
    let mut engine = GenericGraphReductionMachine::<SlotMap<_, _>>::from_tree(parsed)
        .set_mode(ReductionMode::NormalForm);

    engine.reduce();

    assert_eq!(
        format!("{engine}"),
        "S(K(S(S(KS)K)I))(S(K(S(S(KS)K)I))(S(K(S(S(KS)K)I))(S(S(KS)K)I)))"
    );
}

#[test]
fn normal_form_exhaustion() {
    let input = "S(SKSKSSK)(SSKSKSK)";
    let parsed = lexor_reducer::parse(input).unwrap();
    let mut engine = GenericGraphReductionMachine::<SlotMap<_, _>>::from_tree(parsed)
        .set_mode(ReductionMode::WeakHeadNormalForm);

    engine.reduce();

    assert_eq!(format!("{engine}"), input);
}

#[test]
fn ackermann_small() {
    let input = "(S (S (KS) K) (S (S (KS) K) I)) (S (S (KS) K) I) (S (S (KS) K) I)";
    let parsed = lexor_reducer::parse(input).unwrap();
    let mut engine = GenericGraphReductionMachine::<SlotMap<_, _>>::from_tree(parsed)
        .set_mode(ReductionMode::NormalForm);

    engine.reduce();

    assert_eq!(
        format!("{engine}"),
        "S(K(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)I)))(S(K(S(S(KS)K)I))(S(S(KS)K)I))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)I)))(S(K(S(S(KS)K)I))(S(S(KS)K)I)))"
    );
}

#[test]
fn six_factorial() {
    let input = "(S (S (KS) K) (S (K (S (S (KS) K) (S (S (KS) K) (S (S (KS) K) (S (S (KS) K) (S (S (KS) K) I)))))) (S (S (KS) K) (S (S (KS) K) (S (S (KS) K) (S (S (KS) K) (S (S (KS) K) (S (S (KS) K) I)))))))) (S (K (S (S (KS) K) I)) (S (S (KS) K) (S (S (KS) K) (S (S (KS) K) (S (S (KS) K) (S (S (KS) K) I))))))";
    let parsed = lexor_reducer::parse(input).unwrap();
    let mut engine: GenericGraphReductionMachine<SlotMap<_, _>> =
        GenericGraphReductionMachine::from_tree(parsed).set_mode(ReductionMode::NormalForm);

    engine.reduce();

    assert_eq!(
        format!("{engine}"),
        "S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))))))))(S(K(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))))))))(S(K(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))))))))(S(K(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))))))))(S(K(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))(S(K(S(S(KS)K)I))(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)(S(S(KS)K)I)))))))))))))))))"
    );
}
