use super::super::Frac;

#[test]
// from positive u8
fn pos_xxx_u8xx() {
    assert_eq!(
        Frac::new(5, 1),
        Frac::from(5_u8),
    );
}

#[test]
// from positive u16
fn pos_xxx_u16x() {
    assert_eq!(
        Frac::new(5, 1),
        Frac::from(5_u16),
    );
}

#[test]
// from positive u32
fn pos_xxx_u32x() {
    assert_eq!(
        Frac::new(5, 1),
        Frac::from(5_u32),
    );
}

#[test]
// from positive u64
fn pos_xxx_u64x() {
    assert_eq!(
        Frac::new(5, 1),
        Frac::from(5_u64),
    );
}

#[test]
// from positive u128
fn pos_xxx_u128() {
    assert_eq!(
        Frac::new(5, 1),
        Frac::from(5_u128),
    );
}

#[test]
// from positive usize
fn pos_xxx_usiz() {
    assert_eq!(
        Frac::new(5, 1),
        Frac::from(5_usize),
    );
}

#[test]
// from zero u8
fn zer_xxx_u8xx() {
    assert_eq!(
        Frac::new(0, 1),
        Frac::from(0_u8),
    );
}

#[test]
// from zero u16
fn zer_xxx_u16x() {
    assert_eq!(
        Frac::new(0, 1),
        Frac::from(0_u16),
    );
}

#[test]
// from zero u32
fn zer_xxx_u32x() {
    assert_eq!(
        Frac::new(0, 1),
        Frac::from(0_u32),
    );
}

#[test]
// from zero u64
fn zer_xxx_u64x() {
    assert_eq!(
        Frac::new(0, 1),
        Frac::from(0_u64),
    );
}

#[test]
// from zero u128
fn zer_xxx_u128() {
    assert_eq!(
        Frac::new(0, 1),
        Frac::from(0_u128),
    );
}

#[test]
// from zero usize
fn zer_xxx_usiz() {
    assert_eq!(
        Frac::new(0, 1),
        Frac::from(0_usize),
    );
}

#[test]
// from positive i8
fn pos_xxx_i8xx() {
    assert_eq!(
        Frac::new(5, 1),
        Frac::from(5_i8),
    );
}

#[test]
// from positive i16
fn pos_xxx_i16x() {
    assert_eq!(
        Frac::new(5, 1),
        Frac::from(5_i16),
    );
}

#[test]
// from positive i32
fn pos_xxx_i32x() {
    assert_eq!(
        Frac::new(5, 1),
        Frac::from(5_i32),
    );
}

#[test]
// from positive i64
fn pos_xxx_i64x() {
    assert_eq!(
        Frac::new(5, 1),
        Frac::from(5_i64),
    );
}

#[test]
// from positive i128
fn pos_xxx_i128() {
    assert_eq!(
        Frac::new(5, 1),
        Frac::from(5_i128),
    );
}

#[test]
// from positive isize
fn pos_xxx_isiz() {
    assert_eq!(
        Frac::new(5, 1),
        Frac::from(5_isize),
    );
}

#[test]
// from zero i8
fn zer_xxx_i8xx() {
    assert_eq!(
        Frac::new(0, 1),
        Frac::from(0_i8),
    );
}

#[test]
// from zero i16
fn zer_xxx_i16x() {
    assert_eq!(
        Frac::new(0, 1),
        Frac::from(0_i16),
    );
}

#[test]
// from zero i32
fn zer_xxx_i32x() {
    assert_eq!(
        Frac::new(0, 1),
        Frac::from(0_i32),
    );
}

#[test]
// from zero i64
fn zer_xxx_i64x() {
    assert_eq!(
        Frac::new(0, 1),
        Frac::from(0_i64),
    );
}

#[test]
// from zero i128
fn zer_xxx_i128() {
    assert_eq!(
        Frac::new(0, 1),
        Frac::from(0_i128),
    );
}

#[test]
// from zero isize
fn zer_xxx_isiz() {
    assert_eq!(
        Frac::new(0, 1),
        Frac::from(0_isize),
    );
}

#[test]
// from negative i8
fn neg_xxx_i8xx() {
    assert_eq!(
        Frac::new(-5, 1),
        Frac::from(-5_i8),
    );
}

#[test]
// from negative i16
fn neg_xxx_i16x() {
    assert_eq!(
        Frac::new(-5, 1),
        Frac::from(-5_i16),
    );
}

#[test]
// from negative i32
fn neg_xxx_i32x() {
    assert_eq!(
        Frac::new(-5, 1),
        Frac::from(-5_i32),
    );
}

#[test]
// from negative i64
fn neg_xxx_i64x() {
    assert_eq!(
        Frac::new(-5, 1),
        Frac::from(-5_i64),
    );
}

#[test]
// from negative i128
fn neg_xxx_i128() {
    assert_eq!(
        Frac::new(-5, 1),
        Frac::from(-5_i128),
    );
}

#[test]
// from negative isize
fn neg_xxx_isiz() {
    assert_eq!(
        Frac::new(-5, 1),
        Frac::from(-5_isize),
    );
}

#[test]
// from positive integer string
fn str_int_posx() {
    assert_eq!(
        Frac::from(17),
        "17".parse().unwrap(),
    );
}

#[test]
// from negative integer string
fn str_int_negx() {
    assert_eq!(
        Frac::from(-17),
        "-17".parse().unwrap(),
    );
}

#[test]
// from pos pos string
fn str_pos_posx() {
    assert_eq!(
        Frac::new(2, 5),
        "2/5".parse().unwrap(),
    );
}

#[test]
// from pos neg string
fn str_pos_negx() {
    assert_eq!(
        Frac::new(2, -5),
        "2/-5".parse().unwrap(),
    );
}

#[test]
// from neg pos string
fn str_neg_posx() {
    assert_eq!(
        Frac::new(-2, 5),
        "-2/5".parse().unwrap(),
    );
}

#[test]
// from neg neg string
fn str_neg_negx() {
    assert_eq!(
        Frac::new(-2, -5),
        "-2/-5".parse().unwrap(),
    );
}

#[test]
#[should_panic]
// empty string panics
fn str_emp_fail() {
    let _: Frac = "".parse().unwrap();
}

#[test]
#[should_panic]
// multiple slashes in string panics
fn str_mlt_fail() {
    let _: Frac = "3/2/1".parse().unwrap();
}

#[test]
#[should_panic]
// invalid integer integer string panics
fn str_int_intf() {
    let _: Frac = "312d".parse().unwrap();
}

#[test]
#[should_panic]
// invalid integer frac string panics (1)
fn str_int_frcf() {
    let _: Frac = "312d/2".parse().unwrap();
}

#[test]
#[should_panic]
// invalid integer frac string panics (2)
fn str_frc_intf() {
    let _: Frac = "2/321d".parse().unwrap();
}
