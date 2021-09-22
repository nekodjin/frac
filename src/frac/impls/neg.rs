use std::ops;
use super::super::Frac;

impl ops::Neg for Frac {
    type Output = Frac;

    fn neg(self) -> Frac {
        Frac {
            neg: !self.neg,
            ..self
        }
    }
}
