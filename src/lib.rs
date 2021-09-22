use std::fmt;
use std::ops;

#[derive(Debug, Clone)]
pub struct Frac {
    neg: bool,
    num: u128,
    den: u128,
}

// associated functions
impl Frac {
    pub const fn new(num: i128, den: i128) -> Self {
        let neg = is_neg(num) ^ is_neg(den);

        let num = unsign(num);
        let den = unsign(den);

        let gcd = gcd_of(num, den);

        let num = num / gcd;
        let den = den / gcd;

        Frac {
            neg,
            num,
            den,
        }
    }

    pub const fn new_sign(neg: bool, num: u128, den: u128) -> Frac {
        let gcd = gcd_of(num, den);

        let num = num / gcd;
        let den = den / gcd;

        Frac {
            neg,
            num,
            den,
        }
    }
}

// methods
impl Frac {
    pub fn recip(&self) -> Self {
        Frac {
            neg: self.neg,
            num: self.den,
            den: self.num,
        }
    }
}

macro_rules! impl_from_uint {
    ($($T:ty),*) => {$(
        impl From<$T> for Frac {
            fn from(val: $T) -> Frac {
                Frac {
                    neg: false,
                    num: val as u128,
                    den: 1,
                }
            }
        }
    )*};
}

impl_from_uint! { u8, u16, u32, u64, u128 }

macro_rules! impl_from_int {
    ($($T:ty),*) => {$(
        impl From<$T> for Frac {
            fn from(val: $T) -> Frac {
                Frac {
                    neg: is_neg(val as i128),
                    num: unsign(val as i128),
                    den: 1,
                }
            }
        }
    )*};
}

impl_from_int! { i8, i16, i32, i64, i128 }

impl fmt::Display for Frac {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}/{}",
            if self.neg { "-" } else { "" },
            self.num,
            self.den,
        )
    }
}

impl ops::Neg for Frac {
    type Output = Frac;

    fn neg(self) -> Frac {
        Frac {
            neg: !self.neg,
            ..self
        }
    }
}

impl ops::Add for Frac {
    type Output = Frac;

    fn add(self, rhs: Frac) -> Frac {
        if self.neg ^ rhs.neg {
            if self.neg {
                rhs - self
            }
            else {
                self - -rhs
            }
        }
        else {
            Frac::new_sign(
                self.neg,
                self.num * rhs.den + rhs.num * self.den,
                self.den * rhs.den
            )
        }
    }
}

macro_rules! impl_add {
    ($($T:ty),*,) => {$(
        impl ops::Add<$T> for Frac {
            type Output = Frac;

            fn add(self, rhs: $T) -> Frac {
                <Frac as ops::Add>::add(self, rhs.into())
            }
        }

        impl ops::Add<Frac> for $T {
            type Output = Frac;

            fn add(self, rhs: Frac) -> Frac {
                <Frac as ops::Add>::add(self.into(), rhs)
            }
        }
    )*};
}

impl_add! {
    u8, u16, u32, u64, u128,
    i8, i16, i32, i64, i128,
}

impl ops::Sub for Frac {
    type Output = Frac;

    fn sub(self, rhs: Frac) -> Frac {
        if !(self.neg || rhs.neg) {
            if self.num >= rhs.num {
                Frac::new_sign(
                    false,
                    self.num * rhs.den - rhs.num * self.den,
                    self.den * rhs.den
                )
            }
            else {
                Frac::new_sign(
                    true,
                    rhs.num * self.den - self.num * rhs.den,
                    self.den * rhs.den
                )
            }
        }
        else if rhs.neg {
            self + -rhs
        }
        else {
            -(-self + -rhs)
        }
    }
}

macro_rules! impl_sub {
    ($($T:ty),*,) => {$(
        impl ops::Sub<$T> for Frac {
            type Output = Frac;

            fn sub(self, rhs: $T) -> Frac {
                <Frac as ops::Sub>::sub(self, rhs.into())
            }
        }

        impl ops::Sub<Frac> for $T {
            type Output = Frac;

            fn sub(self, rhs: Frac) -> Frac {
                <Frac as ops::Sub>::sub(self.into(), rhs)
            }
        }
    )*};
}

impl_sub! {
    u8, u16, u32, u64, u128,
    i8, i16, i32, i64, i128,
}

impl ops::Mul for Frac {
    type Output = Frac;

    fn mul(self, rhs: Frac) -> Frac {
        Frac::new_sign(
            self.neg ^ rhs.neg,
            self.num * rhs.num,
            self.den * rhs.den,
        )
    }
}

macro_rules! impl_mul {
    ($($T:ty),*,) => {$(
        impl ops::Mul<$T> for Frac {
            type Output = Frac;

            fn mul(self, rhs: $T) -> Frac {
                <Frac as ops::Mul>::mul(self, rhs.into())
            }
        }

        impl ops::Mul<Frac> for $T {
            type Output = Frac;

            fn mul(self, rhs: Frac) -> Frac {
                <Frac as ops::Mul>::mul(self.into(), rhs)
            }
        }
    )*};
}

impl_mul! {
    u8, u16, u32, u64, u128,
    i8, i16, i32, i64, i128,
}

impl ops::Div for Frac {
    type Output = Frac;

    fn div(self, rhs: Frac) -> Frac {
        self * rhs.recip()
    }
}

macro_rules! impl_div {
    ($($T:ty),*,) => {$(
        impl ops::Div<$T> for Frac {
            type Output = Frac;

            fn div(self, rhs: $T) -> Frac {
                <Frac as ops::Div>::div(self, rhs.into())
            }
        }

        impl ops::Div<Frac> for $T {
            type Output = Frac;

            fn div(self, rhs: Frac) -> Frac {
                <Frac as ops::Div>::div(self.into(), rhs)
            }
        }
    )*};
}

impl_div! {
    u8, u16, u32, u64, u128,
    i8, i16, i32, i64, i128,
}

const fn is_neg(n: i128) -> bool {
    0 > n
}

const fn unsign(n: i128) -> u128 {
    n.abs() as u128
}

const fn gcd_of(a: u128, b: u128) -> u128 {
    if b == 0 { a } else { gcd_of(b, a % b) }
}
