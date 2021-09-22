//! Rational Data Type
//!
//! This crate provides the `Frac` (fractional) data type, which represents a
//! rational number.

mod frac;

#[cfg(test)]
mod tests;

pub use crate::frac::Frac;

