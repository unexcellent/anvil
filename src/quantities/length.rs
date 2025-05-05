use std::ops::Mul;

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
pub struct Length {
    mm: f64,
}
impl Length {
    pub fn from_mm(value: f64) -> Self {
        Length { mm: value }
    }
    pub fn mm(&self) -> f64 {
        self.mm
    }
    pub fn from_m(value: f64) -> Self {
        Length { mm: value * 1000. }
    }
    pub fn m(&self) -> f64 {
        self.mm / 1000.
    }

    pub fn min(&self, other: &Self) -> Self {
        Length {
            mm: self.mm.min(other.mm),
        }
    }
    pub fn max(&self, other: &Self) -> Self {
        Length {
            mm: self.mm.max(other.mm),
        }
    }
}

impl Mul<f64> for Length {
    type Output = Length;
    fn mul(self, rhs: f64) -> Length {
        Length { mm: self.mm * rhs }
    }
}
impl Mul<Length> for f64 {
    type Output = Length;
    fn mul(self, rhs: Length) -> Length {
        rhs * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply_with_f64() {
        assert_eq!(Length::from_m(5.) * 4., Length::from_m(20.));
        assert_eq!(4. * Length::from_m(5.), Length::from_m(20.));
    }
}
