use super::super::Frac;

#[test]
fn pos_pos_xxxx() {
    assert_eq!(
        Frac::new(7, 12),
        Frac::new(1, 3) + Frac::new(1, 4),
    );
}

#[test]
fn neg_neg_xxxx() {
    assert_eq!(
        Frac::new(-7, 12),
        Frac::new(-1, 3) + Frac::new(-1, 4),
    );
}

#[test]
fn pos_neg_uflw() {
    assert_eq!(
        Frac::new(1, 12),
        Frac::new(1, 3) + Frac::new(-1, 4),
    );
}

#[test]
fn pos_neg_oflw() {
    assert_eq!(
        Frac::new(-1, 12),
        Frac::new(1, 4) + Frac::new(-1, 3),
    );
}

#[test]
fn neg_pos_uflw() {
    assert_eq!(
        Frac::new(-1, 12),
        Frac::new(-1, 3) + Frac::new(1, 4),
    );
}

#[test]
fn neg_pos_oflw() {
    assert_eq!(
        Frac::new(1, 12),
        Frac::new(-1, 4) + Frac::new(1, 3),
    );
}
