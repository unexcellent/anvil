use std::ops::{Add, Div, Mul, Sub};

use super::into_f64::IntoF64;

/// A physical length (i.e. a distance).
///
/// Length exists to remove ambiguity about distance units, which are not supported by default by
/// major CAD kernels.
#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
pub struct Length {
    mm: f64,
}
impl Length {
    /// Construct a `Length` with a value of zero.
    ///
    /// # Example
    /// ```rust
    /// use anvil::Length;
    ///
    /// let len = Length::zero();
    /// assert_eq!(len.m(), 0.);
    /// ```
    pub fn zero() -> Self {
        Self { mm: 0. }
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
    pub fn from_m<T: IntoF64>(value: T) -> Self {
        Length {
            mm: value.into_f64() * 1000.,
        }
    }
    /// Return the value of this length in millimeters.
    pub fn m(&self) -> f64 {
        self.mm / 1000.
    }
    /// Construct a `Length` from a value of unit centimeters.
    ///
    /// # Example
    /// ```rust
    /// use anvil::Length;
    ///
    /// let len = Length::from_cm(5.4);
    /// assert_eq!(len.mm(), 54.);
    /// ```
    pub fn from_cm<T: IntoF64>(value: T) -> Self {
        Length {
            mm: value.into_f64() * 10.,
        }
    }
    /// Return the value of this length in centimeters.
    pub fn cm(&self) -> f64 {
        self.mm / 10.
    }
    /// Construct a `Length` from a value of unit millimeters.
    ///
    /// # Example
    /// ```rust
    /// use anvil::Length;
    ///
    /// let len = Length::from_mm(5.4);
    /// assert_eq!(len.m(), 0.0054);
    /// ```
    pub fn from_mm<T: IntoF64>(value: T) -> Self {
        Length {
            mm: value.into_f64(),
        }
    }
    /// Return the value of this length in millimeters.
    pub fn mm(&self) -> f64 {
        self.mm
    }

    /// Return the smaller of two lengths.
    ///
    /// # Example
    /// ```rust
    /// use anvil::length;
    ///
    /// let len1 = length!(1 m);
    /// let len2 = length!(2 m);
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
    /// use anvil::length;
    ///
    /// let len1 = length!(1 m);
    /// let len2 = length!(2 m);
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

/// Return true if any length in the input array is zero.
pub fn is_zero(lengths: &[Length]) -> bool {
    for length in lengths {
        if length.mm == 0. {
            return true;
        }
    }
    false
}

/// Macro for simplifying `Length` construction for static values.
///
/// Create a length with the correct unit by invoking `length!([value] [unit])`.
///
/// # Examples
/// ```rust
/// use anvil::{length, Length};
///
/// assert_eq!(length!(5 m), Length::from_m(5));
/// assert_eq!(length!(5.1 m), Length::from_m(5.1));
/// assert_eq!(length!(2 cm), Length::from_cm(2.));
/// assert_eq!(length!(1 mm), Length::from_mm(1));
/// assert_eq!(length!(0), Length::zero());
/// ```
#[macro_export]
macro_rules! length {
    ( 0 ) => {
        $crate::Length::zero()
    };
    ( $val:literal m ) => {
        $crate::Length::from_m($val as f64)
    };
    ( $val:literal cm ) => {
        $crate::Length::from_cm($val as f64)
    };
    ( $val:literal mm ) => {
        $crate::Length::from_mm($val as f64)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(length!(2 m) + length!(3 m), length!(5 m));
    }

    #[test]
    fn subtract() {
        assert_eq!(length!(3 m) - length!(2 m), length!(1 m));
    }

    #[test]
    fn multiply_with_f64() {
        assert_eq!(length!(5 m) * 4., Length::from_m(20.));
        assert_eq!(4. * length!(5 m), Length::from_m(20.));
    }

    #[test]
    fn divide_with_f64() {
        assert_eq!(Length::from_m(6.) / 2., length!(3 m));
    }
}
