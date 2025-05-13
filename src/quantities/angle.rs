use core::f64;
use std::ops::{Add, Div, Mul, Sub};

use super::into_f64::IntoF64;

/// A physical angle (i.e. a distance).
///
/// Angle exists to remove ambiguity about distance units, which are not supported by default by
/// major CAD kernels.
#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
pub struct Angle {
    rad: f64,
}
impl Angle {
    /// Construct a `Angle` with a value of zero.
    ///
    /// # Example
    /// ```rust
    /// use anvil::Angle;
    ///
    /// let angle = Angle::zero();
    /// assert_eq!(angle.deg(), 0.);
    /// ```
    pub fn zero() -> Self {
        Self { rad: 0. }
    }
    /// Construct a `Angle` from a value in radians.
    ///
    /// # Example
    /// ```rust
    /// use core::f64;
    /// use anvil::Angle;
    ///
    /// let angle = Angle::from_rad(f64::consts::PI);
    /// assert_eq!(angle.deg(), 180.);
    /// ```
    pub fn from_rad<T: IntoF64>(value: T) -> Self {
        Angle {
            rad: value.into_f64(),
        }
    }
    /// Return the value of this angle in radians.
    pub fn rad(&self) -> f64 {
        self.rad
    }
    /// Construct a `Angle` from a value in degrees.
    ///
    /// # Example
    /// ```rust
    /// use core::f64;
    /// use anvil::Angle;
    ///
    /// let angle = Angle::from_deg(180);
    /// assert_eq!(angle.rad(), f64::consts::PI);
    /// ```
    pub fn from_deg<T: IntoF64>(value: T) -> Self {
        Angle {
            rad: value.into_f64() / 360. * f64::consts::TAU,
        }
    }
    /// Return the value of this angle in degrees.
    pub fn deg(&self) -> f64 {
        self.rad / f64::consts::TAU * 360.
    }

    /// Return the smaller of two angles.
    ///
    /// # Example
    /// ```rust
    /// use anvil::Angle;
    ///
    /// let angle1 = Angle::from_rad(1.);
    /// let angle2 = Angle::from_rad(2.);
    /// assert_eq!(angle1.min(&angle2), angle1);
    /// assert_eq!(angle2.min(&angle1), angle1);
    /// ```
    pub fn min(&self, other: &Self) -> Self {
        Angle {
            rad: self.rad.min(other.rad),
        }
    }
    /// Return the larger of two lengths.
    ///
    /// # Example
    /// ```rust
    /// use anvil::Angle;
    ///
    /// let angle1 = Angle::from_rad(1);
    /// let angle2 = Angle::from_rad(2);
    /// assert_eq!(angle1.max(&angle2), angle2);
    /// assert_eq!(angle2.max(&angle1), angle2);
    /// ```
    pub fn max(&self, other: &Self) -> Self {
        Angle {
            rad: self.rad.max(other.rad),
        }
    }
}

impl Add<Angle> for Angle {
    type Output = Angle;
    fn add(self, other: Angle) -> Angle {
        Angle {
            rad: self.rad + other.rad,
        }
    }
}

impl Sub<Angle> for Angle {
    type Output = Angle;
    fn sub(self, other: Angle) -> Angle {
        Angle {
            rad: self.rad - other.rad,
        }
    }
}

impl Mul<f64> for Angle {
    type Output = Angle;
    fn mul(self, other: f64) -> Angle {
        Angle {
            rad: self.rad * other,
        }
    }
}

impl Mul<Angle> for f64 {
    type Output = Angle;
    fn mul(self, other: Angle) -> Angle {
        other * self
    }
}

impl Div<f64> for Angle {
    type Output = Angle;
    fn div(self, other: f64) -> Angle {
        Angle {
            rad: self.rad / other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(
            Angle::from_rad(2.) + Angle::from_rad(3.),
            Angle::from_rad(5.)
        );
    }

    #[test]
    fn subtract() {
        assert_eq!(
            Angle::from_rad(3.) - Angle::from_rad(2.),
            Angle::from_rad(1.)
        );
    }

    #[test]
    fn multiply_with_f64() {
        assert_eq!(Angle::from_rad(5.) * 4., Angle::from_rad(20.));
        assert_eq!(4. * Angle::from_rad(5.), Angle::from_rad(20.));
    }

    #[test]
    fn divide_with_f64() {
        assert_eq!(Angle::from_rad(6.) / 2., Angle::from_rad(3.));
    }
}
