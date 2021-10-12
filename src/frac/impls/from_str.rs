use std::str::FromStr;
use super::super::Frac;

impl FromStr for Frac {
    type Err = ();

    fn from_str(s: &str) -> Result<Frac, ()> {
        let parts: Vec<&str> = s.split('/').collect();

        if parts.len() == 1 {
            let val: i128 = s.parse().or(Err(()))?;
            return Ok(Frac::from(val));
        }

        if parts.len() == 2 {
            let num = parts[0].parse().or(Err(()))?;
            let den = parts[1].parse().or(Err(()))?;

            return Ok(Frac::new(num, den));
        }

        Err(())
    }
}
