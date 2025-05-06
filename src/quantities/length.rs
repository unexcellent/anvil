use std::ops::{Add, Div, Mul, Sub};

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

impl Add<Length> for Length {
    type Output = Length;
    fn add(self, other: Length) -> Length {
        Length {
            mm: self.mm + other.mm,
        }
    }
}

impl Sub<Length> for Length {
    type Output = Length;
    fn sub(self, other: Length) -> Length {
        Length {
            mm: self.mm - other.mm,
        }
    }
}

impl Mul<f64> for Length {
    type Output = Length;
    fn mul(self, other: f64) -> Length {
        Length {
            mm: self.mm * other,
        }
    }
}

impl Mul<Length> for f64 {
    type Output = Length;
    fn mul(self, other: Length) -> Length {
        other * self
    }
}

impl Div<f64> for Length {
    type Output = Length;
    fn div(self, other: f64) -> Length {
        Length {
            mm: self.mm / other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(Length::from_m(2.) + Length::from_m(3.), Length::from_m(5.));
    }

    #[test]
    fn subtract() {
        assert_eq!(Length::from_m(3.) - Length::from_m(2.), Length::from_m(1.));
    }

    #[test]
    fn multiply_with_f64() {
        assert_eq!(Length::from_m(5.) * 4., Length::from_m(20.));
        assert_eq!(4. * Length::from_m(5.), Length::from_m(20.));
    }

    #[test]
    fn divide_with_f64() {
        assert_eq!(Length::from_m(6.) / 2., Length::from_m(3.));
    }
}
