use super::super::Frac;

#[test]
// (+x) - (+y) == (+z)
fn pos_pos_uflw() {
    assert_eq!(
        Frac::new(3, 8),
        Frac::new(1, 2) - Frac::new(1, 8),
    );
}

#[test]
// (+x) - (+y) == (-z)
fn pos_pos_oflw() {
    assert_eq!(
        Frac::new(-3, 8),
        Frac::new(1, 8) - Frac::new(1, 2),
    );
}

#[test]
// (-x) - (-y) == (-z)
fn neg_neg_uflw() {
    assert_eq!(
        Frac::new(-1, 4),
        Frac::new(-3, 2) - Frac::new(-20, 16),
    );
}

#[test]
// (-x) - (-y) == (+z)
fn neg_neg_oflw() {
    assert_eq!(
        Frac::new(1, 4),
        Frac::new(-20, 16) - Frac::new(-3, 2),
    );
}

#[test]
// 0 - 0 == 0
fn zer_zer_xxxx() {
    assert_eq!(
        Frac::from(0),
        Frac::from(0) - 0,
    );
}

#[test]
// (+x) - (-y) == (+z)
fn pos_neg_xxxx() {
    assert_eq!(
        Frac::new(5, 7),
        Frac::new(3, 14) - Frac::new(-1, 2),
    );
}

#[test]
// (+x) - 0 == (+x)
fn pos_zer_xxxx() {
    assert_eq!(
        Frac::new(1, 5),
        Frac::new(1, 5) - 0,
    );
}

#[test]
// (-x) - (+y) == (-z)
fn neg_pos_xxxx() {
    assert_eq!(
        Frac::new(-5, 7),
        Frac::new(-3, 14) - Frac::new(1, 2),
    );
}

#[test]
// (-x) - 0 == (-x)
fn neg_zer_xxxx() {
    assert_eq!(
        Frac::new(-1, 5),
        Frac::new(-1, 5) - 0,
    );
}

#[test]
// 0 - (+x) == (-x)
fn zer_pos_xxxx() {
    assert_eq!(
        Frac::new(-1, 5),
        0 - Frac::new(1, 5),
    );
}

#[test]
// 0 - (-x) == (+x)
fn zer_neg_xxxx() {
    assert_eq!(
        Frac::new(1, 5),
        0 - Frac::new(-1, 5),
    );
}
