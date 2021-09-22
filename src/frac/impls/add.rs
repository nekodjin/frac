use std::ops;
use super::super::Frac;

impl ops::Add for Frac {
    type Output = Frac;

    fn add(self, rhs: Frac) -> Frac {
        if self.neg ^ rhs.neg {
            if self.neg {
                rhs - -self
            }
            else {
                self - -rhs
            }
        }
        else {
            Frac::new_sign(
                self.neg,
                self.num * rhs.den + rhs.num * self.den,
                self.den * rhs.den,
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
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
}
