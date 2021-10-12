use super::super::Frac;

#[test]
// (+x) * (+y) == (+z)
fn pos_pos_xxxx() {
    assert_eq!(
        Frac::new(11, 15),
        Frac::new(1, 3) * Frac::new(11, 5),
    );
}

#[test]
// (-x) * (-y) == (+z)
fn neg_neg_xxxx() {
    assert_eq!(
        Frac::new(11, 15),
        Frac::new(-1, 3) * Frac::new(-11, 5),
    )
}

#[test]
// 0 * 0 == 0
fn zer_zer_xxxx() {
    assert_eq!(
        Frac::from(0),
        Frac::from(0) * 0,
    );
}

#[test]
// (+x) * (-y) == (-z)
fn pos_neg_xxxx() {
    assert_eq!(
        Frac::new(-11, 15),
        Frac::new(1, 3) * Frac::new(-11, 5),
    );
}

#[test]
// (+x) * 0 == 0
fn pos_zer_xxxx() {
    assert_eq!(
        Frac::from(0),
        Frac::new(1, 5) * 0,
    );
}

#[test]
// (-x) * (+y) == (-z)
fn neg_pos_xxxx() {
    assert_eq!(
        Frac::new(-11, 15),
        Frac::new(-1, 3) * Frac::new(11, 5),
    );
}

#[test]
// (-x) * 0 == 0
fn neg_zer_xxxx() {
    assert_eq!(
        Frac::from(0),
        Frac::new(-1, 5) * 0,
    );
}

#[test]
// 0 * (+x) == 0
fn zer_pos_xxxx() {
    assert_eq!(
        Frac::from(0),
        0 * Frac::new(1, 5),
    );
}

#[test]
// 0 * (-x) == 0
fn zer_neg_xxxx() {
    assert_eq!(
        Frac::from(0),
        0 * Frac::new(-1, 5),
    );
}
