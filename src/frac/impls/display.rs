use std::fmt;
use super::super::Frac;

impl fmt::Display for Frac {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}/{}",
            if self.neg { "-" } else { "" },
            self.num,
            self.den,
        )
    }
}
