use std::cmp;
use super::super::Frac;

impl cmp::PartialOrd for Frac {
    fn partial_cmp(&self, rhs: &Frac) -> Option<cmp::Ordering> {
        if self.den == 0 || rhs.den == 0 {
            return None;
        }

        if self.neg ^ rhs.neg {
            if self.neg {
                return Some(cmp::Ordering::Less);
            }
            else {
                return Some(cmp::Ordering::Greater);
            }
        }

        let lhsn = self.num * rhs.den;
        let rhsn = self.den * rhs.num;

        if self.neg {
            return rhsn.partial_cmp(&lhsn);
        }
        else {
            return lhsn.partial_cmp(&rhsn);
        }
    }
}
