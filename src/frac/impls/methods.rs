use super::super::Frac;

/// Instance Methods
impl Frac {
    /// Reciprocal
    ///
    /// Instantiates a new fraction equivalent to the reciprocal of the callee.
    ///
    /// Examples:
    /// ```
    /// use frac::Frac;
    ///
    /// let f = Frac::from(-12);
    /// println!("{}", f.recip()); // -1/12
    /// ```
    pub fn recip(&self) -> Self {
        Frac {
            neg: self.neg,
            num: self.den,
            den: self.num,
        }
    }
}
