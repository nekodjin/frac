use super::super::Frac;

#[test]
// recip(+x/y) == +y/x
fn pos_xxx_xxxx() {
    assert_eq!(
        Frac::new(4, 5),
        Frac::new(5, 4).recip(),
    );
}

#[test]
// recip(-x/y) == -y/x
fn neg_xxx_xxxx() {
    assert_eq!(
        Frac::new(-4, 5),
        Frac::new(-5, 4).recip(),
    );
}
