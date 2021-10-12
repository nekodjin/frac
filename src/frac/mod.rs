mod impls;

/// Represents a rational number
#[derive(Debug, Copy, Clone)]
pub struct Frac {
    neg: bool,
    num: u128,
    den: u128,
}
