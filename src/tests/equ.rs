use super::super::Frac;

#[test]
// (+x) == (+x)
fn pos_pos_equl() {
    assert_eq!(
        Frac::new(12, 14),
        Frac::new(12, 14),
    );
}

#[test]
// (+x) != (+y)
fn pos_pos_nequ() {
    assert_ne!(
        Frac::new(2, 3),
        Frac::new(4, 5),
    );
}

#[test]
// (-x) == (-x)
fn neg_neg_equl() {
    assert_eq!(
        Frac::new(-7, 5),
        Frac::new(-7, 5),
    );
}

#[test]
// (-x) != (-y)
fn neg_neg_nequ() {
    assert_ne!(
        Frac::new(-4, 3),
        Frac::new(-5, 4),
    );
}

#[test]
// 0 == 0
fn zer_zer_equl() {
    assert_eq!(
        Frac::from(0),
        Frac::from(0),
    );
}

#[test]
// -0 == -0
fn nzr_nzr_equl() {
    assert_eq!(
        -Frac::from(0),
        -Frac::from(0),
    );
}

#[test]
// 0 == -0
fn zer_nzr_equl() {
    assert_eq!(
        Frac::from(0),
        -Frac::from(0),
    );
}

#[test]
// -0 == 0
fn nzr_zer_equl() {
    assert_eq!(
        -Frac::from(0),
        Frac::from(0),
    );
}

#[test]
// (+x) != (-x)
fn pos_neg_nequ() {
    assert_ne!(
        Frac::new( 4, 5),
        Frac::new(-4, 5),
    );
}

#[test]
// (-x) != (+x)
fn neg_pos_nequ() {
    assert_ne!(
        Frac::new(-4, 5),
        Frac::new( 4, 5),
    );
}

#[test]
// (+x)/0 != (+x)/0
fn pos_zdn_nequ() {
    assert_ne!(
        Frac::new(1, 0),
        Frac::new(1, 0),
    );
}

#[test]
// (-x) != (-x)/0
fn neg_zdn_nequ() {
    assert_ne!(
        Frac::new(-1, 0),
        Frac::new(-1, 0),
    );
}
