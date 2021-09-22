use std::ops;
use super::super::Frac;

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
