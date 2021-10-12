use super::super::Frac;

#[test]
// (+x) / (+y) == (+z)
fn pos_pos_xxxx() {
    assert_eq!(
        Frac::new(20, 21),
        Frac::new(4, 3) / Frac::new(7, 5),
    );
}

#[test]
// (-x) / (-y) == (+z)
fn neg_neg_xxxx() {
    assert_eq!(
        Frac::new(20, 21),
        Frac::new(-4, 3) / Frac::new(-7, 5),
    );
}

// #[test]
// 0 / 0 == undefined
fn zer_zer_xxxx() {
    todo!();
}

#[test]
// (+x) / (-y) == (-z)
fn pos_neg_xxxx() {
    assert_eq!(
        Frac::new(-20, 21),
        Frac::new(4, 3) / Frac::new(-7, 5),
    );
}

// #[test]
// (+x) / 0 == undefined
fn pos_zer_xxxx() {
    todo!();
}

#[test]
// (-x) / (+y) == (-z)
fn neg_pos_xxxx() {
    assert_eq!(
        Frac::new(-20, 21),
        Frac::new(-4, 3) / Frac::new(7, 5),
    );
}

// #[test]
// (-x) / 0 == undefined
fn neg_zer_xxxx() {
    todo!();
}

#[test]
// 0 / (+x) == 0
fn zer_pos_xxxx() {
    assert_eq!(
        Frac::from(0),
        0 / Frac::from(1),
    );
}

#[test]
// 0 / (-x) == 0
fn zer_neg_xxxx() {
    assert_eq!(
        Frac::from(0),
        0 / Frac::from(-1),
    );
}
