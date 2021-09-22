use super::super::Frac;

#[test]
fn pos_pos_uflw() {
    assert_eq!(
        Frac::new(3, 8),
        Frac::new(1, 2) - Frac::new(1, 8),
    );
}

#[test]
fn pos_pos_oflw() {
    assert_eq!(
        Frac::new(-3, 8),
        Frac::new(1, 8) - Frac::new(1, 2),
    );
}

#[test]
fn neg_neg_uflw() {
    assert_eq!(
        Frac::new(-1, 4),
        Frac::new(-3, 2) - Frac::new(-20, 16),
    );
}

#[test]
fn neg_neg_oflw() {
    assert_eq!(
        Frac::new(1, 4),
        Frac::new(-20, 16) - Frac::new(-3, 2),
    );
}

#[test]
fn pos_neg_xxxx() {
    assert_eq!(
        Frac::new(5, 7),
        Frac::new(3, 14) - Frac::new(-1, 2),
    );
}

#[test]
fn neg_pos_xxxx() {
    assert_eq!(
        Frac::new(-5, 7),
        Frac::new(-3, 14) - Frac::new(1, 2),
    );
}
