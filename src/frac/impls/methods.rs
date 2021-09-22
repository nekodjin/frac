use super::super::Frac;

impl Frac {
    /// Reciprocal
    ///
    /// Instantiates a new fraction equivalent to the reciprocal of the callee.
    ///
    /// Examples:
    /// ```
    /// use frac::Frac;
    ///
    /// let f: Frac = (-12).into();
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
