use super::super::Frac;

macro_rules! is_neg {
    ($x:expr) => { $x < 0 };
}

macro_rules! unsign {
    ($x:expr) => {
        (if is_neg!($x) { -$x } else { $x }) as u128
    };
}

impl Frac {
    pub const fn new(num: i128, den: i128) -> Self {
        let neg = is_neg!(num) ^ is_neg!(den);

        let num = unsign!(num);
        let den = unsign!(den);

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

const fn gcd_of(a: u128, b: u128) -> u128 {
    if b == 0 { a } else { gcd_of(b, a % b) }
}
