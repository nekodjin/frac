use super::super::Frac;

#[test]
fn pos_pos_xxxx() {
    assert_eq!(
        Frac::new(11, 15),
        Frac::new(1, 3) * Frac::new(11, 5),
    );
}

#[test]
fn neg_neg_xxxx() {
    assert_eq!(
        Frac::new(11, 15),
        Frac::new(-1, 3) * Frac::new(-11, 5),
    )
}

#[test]
fn pos_neg_xxxx() {
    assert_eq!(
        Frac::new(-11, 15),
        Frac::new(1, 3) * Frac::new(-11, 5),
    );
}

#[test]
fn neg_pos_xxxx() {
    assert_eq!(
        Frac::new(-11, 15),
        Frac::new(-1, 3) * Frac::new(11, 5),
    );
}
