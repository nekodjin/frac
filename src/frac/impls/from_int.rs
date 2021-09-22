use super::super::Frac;

macro_rules! is_neg {
    ($x:expr) => { $x < 0 };
}

macro_rules! unsign {
    ($x:expr) => {
        (if is_neg!($x) { -$x } else { $x }) as u128
    }
}

macro_rules! impl_from_int {
    ($($T:ty),*) => {$(
        impl From<$T> for Frac {
            fn from(val: $T) -> Frac {
                Frac {
                    neg: is_neg!(val as i128),
                    num: unsign!(val as i128),
                    den: 1,
                }
            }
        }
    )*};
}

impl_from_int! { i8, i16, i32, i64, i128 }
