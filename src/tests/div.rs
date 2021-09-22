use super::super::Frac;

#[test]
fn pos_pos_xxxx() {
    assert_eq!(
        Frac::new(20, 21),
        Frac::new(4, 3) / Frac::new(7, 5),
    );
}

#[test]
fn neg_neg_xxxx() {
    assert_eq!(
        Frac::new(20, 21),
        Frac::new(-4, 3) / Frac::new(-7, 5),
    );
}

#[test]
fn pos_neg_xxxx() {
    assert_eq!(
        Frac::new(-20, 21),
        Frac::new(4, 3) / Frac::new(-7, 5),
    );
}

#[test]
fn neg_pos_xxxx() {
    assert_eq!(
        Frac::new(-20, 21),
        Frac::new(-4, 3) / Frac::new(7, 5),
    );
}
