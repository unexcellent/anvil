use std::ops::{Add, Div, Mul, Sub};

use cxx::UniquePtr;
use opencascade_sys::ffi;

use crate::Length;

/// A location in three-dimensional space.
#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
pub struct Point3D {
    pub x: Length,
    pub y: Length,
    pub z: Length,
}
impl Point3D {
    pub(crate) fn to_occt_point(self) -> UniquePtr<ffi::gp_Pnt> {
        ffi::new_point(self.x.m(), self.y.m(), self.z.m())
    }

    /// The origin point at the position x=0, y=0, z=0.
    pub fn origin() -> Self {
        Self::from_mm(0., 0., 0.)
    }

    /// Construct a `Point3D` from its component lengths.
    pub fn new(x: Length, y: Length, z: Length) -> Self {
        Point3D { x, y, z }
    }
    /// Construct a `Point3D` from the millimeter length values directly.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Length, Point3D};
    ///
    /// let point = Point3D::from_mm(1., 2., 3.);
    /// assert_eq!(point.x, Length::from_mm(1.));
    /// assert_eq!(point.y, Length::from_mm(2.));
    /// assert_eq!(point.z, Length::from_mm(3.));
    /// ```
    pub fn from_mm(x: f64, y: f64, z: f64) -> Self {
        Point3D {
            x: Length::from_mm(x),
            y: Length::from_mm(y),
            z: Length::from_mm(z),
        }
    }
    /// Construct a `Point3D` from the meter length values directly.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Length, Point3D};
    ///
    /// let point = Point3D::from_m(1., 2., 3.);
    /// assert_eq!(point.x, Length::from_m(1.));
    /// assert_eq!(point.y, Length::from_m(2.));
    /// assert_eq!(point.z, Length::from_m(3.));
    /// ```
    pub fn from_m(x: f64, y: f64, z: f64) -> Self {
        Point3D {
            x: Length::from_m(x),
            y: Length::from_m(y),
            z: Length::from_m(z),
        }
    }
}

impl Add<Point3D> for Point3D {
    type Output = Point3D;
    fn add(self, other: Point3D) -> Point3D {
        Point3D::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub<Point3D> for Point3D {
    type Output = Point3D;
    fn sub(self, other: Point3D) -> Point3D {
        Point3D::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f64> for Point3D {
    type Output = Point3D;
    fn mul(self, other: f64) -> Point3D {
        Point3D::new(self.x * other, self.y * other, self.z * other)
    }
}
impl Mul<Point3D> for f64 {
    type Output = Point3D;
    fn mul(self, other: Point3D) -> Point3D {
        other * self
    }
}

impl Div<f64> for Point3D {
    type Output = Point3D;
    fn div(self, other: f64) -> Point3D {
        Point3D::new(self.x / other, self.y / other, self.z / other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let point1 = Point3D::from_m(1., 2., 3.);
        let point2 = Point3D::from_m(4., 5., 6.);

        assert_eq!(point1 + point2, Point3D::from_m(5., 7., 9.));
    }

    #[test]
    fn substract() {
        let point1 = Point3D::from_m(1., 2., 3.);
        let point2 = Point3D::from_m(4., 5., 6.);

        assert_eq!(point2 - point1, Point3D::from_m(3., 3., 3.));
    }

    #[test]
    fn multiply() {
        assert_eq!(
            Point3D::from_m(1., 2., 3.) * 4.,
            Point3D::from_m(4., 8., 12.)
        );
        assert_eq!(
            4. * Point3D::from_m(1., 2., 3.),
            Point3D::from_m(4., 8., 12.)
        );
    }

    #[test]
    fn divide() {
        assert_eq!(
            Point3D::from_m(4., 8., 12.) / 4.,
            Point3D::from_m(1., 2., 3.)
        );
    }
}
