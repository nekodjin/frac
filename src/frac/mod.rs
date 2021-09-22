mod impls;

#[derive(Debug, Clone)]
pub struct Frac {
    neg: bool,
    num: u128,
    den: u128,
}
