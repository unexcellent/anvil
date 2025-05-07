use std::ops::{Add, Div, Mul, Sub};

use crate::Length;

/// A location in two-dimensional space.
#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
pub struct Point2D {
    pub x: Length,
    pub y: Length,
}
impl Point2D {
    /// The origin point at the position x=0 and y=0.
    pub fn origin() -> Self {
        Self::from_mm(0., 0.)
    }

    /// Construct a `Point2D` from its component lengths.
    pub fn new(x: Length, y: Length) -> Self {
        Point2D { x, y }
    }
    /// Construct a `Point2D` from the millimeter length values directly.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Length, Point2D};
    ///
    /// let point = Point2D::from_mm(1., 2.);
    /// assert_eq!(point.x, Length::from_mm(1.));
    /// assert_eq!(point.y, Length::from_mm(2.));
    /// ```
    pub fn from_mm(x: f64, y: f64) -> Self {
        Point2D {
            x: Length::from_mm(x),
            y: Length::from_mm(y),
        }
    }
    /// Construct a `Point2D` from the meter length values directly.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Length, Point2D};
    ///
    /// let point = Point2D::from_m(1., 2.);
    /// assert_eq!(point.x, Length::from_m(1.));
    /// assert_eq!(point.y, Length::from_m(2.));
    /// ```
    pub fn from_m(x: f64, y: f64) -> Self {
        Point2D {
            x: Length::from_m(x),
            y: Length::from_m(y),
        }
    }
}

impl Add<Point2D> for Point2D {
    type Output = Point2D;
    fn add(self, other: Point2D) -> Point2D {
        Point2D::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub<Point2D> for Point2D {
    type Output = Point2D;
    fn sub(self, other: Point2D) -> Point2D {
        Point2D::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f64> for Point2D {
    type Output = Point2D;
    fn mul(self, other: f64) -> Point2D {
        Point2D::new(self.x * other, self.y * other)
    }
}
impl Mul<Point2D> for f64 {
    type Output = Point2D;
    fn mul(self, other: Point2D) -> Point2D {
        other * self
    }
}

impl Div<f64> for Point2D {
    type Output = Point2D;
    fn div(self, other: f64) -> Point2D {
        Point2D::new(self.x / other, self.y / other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let point1 = Point2D::from_m(1., 2.);
        let point2 = Point2D::from_m(4., 5.);

        assert_eq!(point1 + point2, Point2D::from_m(5., 7.));
    }

    #[test]
    fn substract() {
        let point1 = Point2D::from_m(1., 2.);
        let point2 = Point2D::from_m(4., 5.);

        assert_eq!(point2 - point1, Point2D::from_m(3., 3.));
    }

    #[test]
    fn multiply() {
        assert_eq!(Point2D::from_m(1., 2.) * 4., Point2D::from_m(4., 8.));
        assert_eq!(4. * Point2D::from_m(1., 2.), Point2D::from_m(4., 8.));
    }

    #[test]
    fn divide() {
        assert_eq!(Point2D::from_m(4., 8.) / 4., Point2D::from_m(1., 2.));
    }
}
