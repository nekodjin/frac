use super::super::Frac;

#[test]
// (+x) + (+y) == (+z)
fn pos_pos_xxxx() {
    assert_eq!(
        Frac::new(7, 12),
        Frac::new(1, 3) + Frac::new(1, 4),
    );
}

#[test]
// (-x) + (-y) == (-z)
fn neg_neg_xxxx() {
    assert_eq!(
        Frac::new(-7, 12),
        Frac::new(-1, 3) + Frac::new(-1, 4),
    );
}

#[test]
// 0 + 0 == 0
fn zer_zer_xxxx() {
    assert_eq!(
        Frac::from(0),
        Frac::from(0) + 0,
    )
}

#[test]
// (+x) + (-y) == (+z)
fn pos_neg_uflw() {
    assert_eq!(
        Frac::new(1, 12),
        Frac::new(1, 3) + Frac::new(-1, 4),
    );
}

#[test]
// (+x) + (-y) == (-z)
fn pos_neg_oflw() {
    assert_eq!(
        Frac::new(-1, 12),
        Frac::new(1, 4) + Frac::new(-1, 3),
    );
}

#[test]
// (+x) + 0 == (+x)
fn pos_zer_xxxx() {
    assert_eq!(
        Frac::new(1, 5),
        Frac::new(1, 5) + 0,
    );
}

#[test]
// (-x) + (+y) == (-z)
fn neg_pos_uflw() {
    assert_eq!(
        Frac::new(-1, 12),
        Frac::new(-1, 3) + Frac::new(1, 4),
    );
}

#[test]
// (-x) + (+y) == (+z)
fn neg_pos_oflw() {
    assert_eq!(
        Frac::new(1, 12),
        Frac::new(-1, 4) + Frac::new(1, 3),
    );
}

#[test]
// (-x) + 0 == (-x)
fn neg_zer_xxxx() {
    assert_eq!(
        Frac::new(-1, 5),
        Frac::new(-1, 5) + 0,
    )
}

#[test]
// 0 + (+x) == (+x)
fn zer_pos_xxxx() {
    assert_eq!(
        Frac::new(1, 5),
        0 + Frac::new(1, 5),
    );
}

#[test]
// 0 + (-x) == (-x)
fn zer_neg_xxxx() {
    assert_eq!(
        Frac::new(-1, 5),
        0 + Frac::new(-1, 5),
    );
}
