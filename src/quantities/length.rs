use std::ops::{Add, Div, Mul, Sub};

/// A physical length (i.e. a distance).
///
/// Length exists to remove ambiguity about distance units, which are not supported by default by
/// major CAD kernels.
#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
pub struct Length {
    mm: f64,
}
impl Length {
    /// Construct a `Length` from a value of unit millimeters.
    ///
    /// # Example
    /// ```rust
    /// use anvil::Length;
    ///
    /// let len = Length::from_mm(5.4);
    /// assert_eq!(len.m(), 0.0054);
    /// ```
    pub fn from_mm(value: f64) -> Self {
        Length { mm: value }
    }
    /// Return the value of this length in millimeters.
    pub fn mm(&self) -> f64 {
        self.mm
    }
    /// Construct a `Length` from a value of unit meters.
    ///
    /// # Example
    /// ```rust
    /// use anvil::Length;
    ///
    /// let len = Length::from_m(3.2);
    /// assert_eq!(len.mm(), 3200.);
    /// ```
    pub fn from_m(value: f64) -> Self {
        Length { mm: value * 1000. }
    }
    /// Return the value of this length in millimeters.
    pub fn m(&self) -> f64 {
        self.mm / 1000.
    }

    /// Return the smaller of two lengths.
    ///
    /// # Example
    /// ```rust
    /// use anvil::Length;
    ///
    /// let len1 = Length::from_m(1.);
    /// let len2 = Length::from_m(2.);
    /// assert_eq!(len1.min(&len2), len1);
    /// assert_eq!(len2.min(&len1), len1);
    /// ```
    pub fn min(&self, other: &Self) -> Self {
        Length {
            mm: self.mm.min(other.mm),
        }
    }
    /// Return the larger of two lengths.
    ///
    /// # Example
    /// ```rust
    /// use anvil::Length;
    ///
    /// let len1 = Length::from_m(1.);
    /// let len2 = Length::from_m(2.);
    /// assert_eq!(len1.max(&len2), len2);
    /// assert_eq!(len2.max(&len1), len2);
    /// ```
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
