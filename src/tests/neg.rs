use super::super::Frac;

#[test]
// -(+x) == (-x)
fn pos_xxx_xxxx() {
    assert_eq!(
        -Frac::new(12, 36),
        Frac::new(-4, 12),
    );
}

#[test]
// -(-x) == (+x)
fn neg_xxx_xxxx() {
    assert_eq!(
        -Frac::new(-17, 20),
        Frac::new(34, 40),
    );
}
