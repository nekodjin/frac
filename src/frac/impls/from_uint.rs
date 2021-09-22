use super::super::Frac;

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
