use std::fmt;
use super::super::Frac;

impl fmt::Display for Frac {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let den_part = format!("/{}", self.den);

        write!(f, "{}{}{}",
            if self.neg { "-" } else { "" },
            self.num,
            if self.den == 1 { "" } else { &den_part },
        )
    }
}
