use super::super::Frac;

impl Frac {
    pub fn recip(&self) -> Self {
        Frac {
            neg: self.neg,
            num: self.den,
            den: self.num,
        }
    }
}
