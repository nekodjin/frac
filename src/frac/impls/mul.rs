use std::ops;
use super::super::Frac;

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
