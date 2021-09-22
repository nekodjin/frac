use std::cmp;
use super::super::Frac;

impl cmp::PartialEq for Frac {
    fn eq(&self, rhs: &Frac) -> bool {
        (self.den != 0 && rhs.den != 0)
            &&
        (self.neg == rhs.neg)
            &&
        (self.num == rhs.num)
            &&
        (self.den == rhs.den)
    }
}
