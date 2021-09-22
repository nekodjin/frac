use std::ops;
use super::super::Frac;

impl ops::Sub for Frac {
    type Output = Frac;

    fn sub(self, rhs: Frac) -> Frac {
        if !(self.neg || rhs.neg) {
            let lhsn = self.num * rhs.den;
            let rhsn = self.den * rhs.num;

            if lhsn >= rhsn {
                Frac::new_sign(
                    false,
                    lhsn - rhsn,
                    self.den * rhs.den
                )
            }
            else {
                Frac::new_sign(
                    true,
                    rhsn - lhsn,
                    self.den * rhs.den
                )
            }
        }
        else {
            -rhs + self
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
