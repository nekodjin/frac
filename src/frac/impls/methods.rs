use super::super::Frac;

impl Frac {
    /// Reciprocal
    ///
    /// Instantiates a new fraction equivalent to the reciprocal of the callee.
    ///
    /// Examples:
    /// ```
    /// let a: Frac = -12.into();
    /// println!("{}", a.recip()); // -1/12
    /// ```
    pub fn recip(&self) -> Self {
        Frac {
            neg: self.neg,
            num: self.den,
            den: self.num,
        }
    }
}
