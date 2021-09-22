use super::super::Frac;

#[test]
fn xxx_xxx_same() {
    assert_eq!(
        Frac::new( 4,  5),
        Frac::new(-4, -5),
    );
}

#[test]
fn xxx_xxx_diff() {
    assert_eq!(
        Frac::new(-4,  5),
        Frac::new( 4, -5),
    );
}

#[test]
fn xxx_xxx_smpl() {
    assert_eq!(
        Frac::new(30, 35),
        Frac::new(42, 49),
    );
}
