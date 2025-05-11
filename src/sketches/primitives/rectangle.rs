use crate::{Length, Path, Point2D, Sketch};

/// Builder for a rectangular `Sketch`.
///
/// While the `Rectangle` struct itself is not used, its constructor methods like
/// `Rectangle::from_dim()` can be used to build this primitive `Sketch`.
#[derive(Debug, PartialEq, Clone)]
pub struct Rectangle;
impl Rectangle {
    /// Construct a centered rectangular `Sketch` from the x and y dimensions.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Length, Point2D, Rectangle};
    ///
    /// let rect = Rectangle::from_dim(Length::from_m(1.), Length::from_m(1.));
    /// assert_eq!(rect.area(), 1.);
    /// assert_eq!(rect.center(), Ok(Point2D::origin()));
    /// ```
    pub fn from_dim(x: Length, y: Length) -> Sketch {
        let corner1 = Point2D {
            x: x * -0.5,
            y: y * -0.5,
        };
        let corner2 = Point2D {
            x: x * 0.5,
            y: y * 0.5,
        };
        Self::from_corners(corner1, corner2)
    }

    /// Construct a rectangular `Sketch` from its corner locations.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Point2D, Rectangle};
    ///
    /// let rect = Rectangle::from_corners(Point2D::origin(), Point2D::from_m(2., 2.));
    /// assert_eq!(rect.area(), 4.);
    /// ```
    pub fn from_corners(corner1: Point2D, corner2: Point2D) -> Sketch {
        Path::at(corner1)
            .line_to(Point2D::new(corner2.x, corner1.y))
            .line_to(corner2)
            .line_to(Point2D::new(corner1.x, corner2.y))
            .close()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_dim_empty() {
        assert_eq!(
            Rectangle::from_dim(Length::from_m(0.), Length::from_m(1.)),
            Sketch::empty()
        );
        assert_eq!(
            Rectangle::from_dim(Length::from_m(1.), Length::from_m(0.)),
            Sketch::empty()
        );
    }

    #[test]
    fn from_corners_empty() {
        assert_eq!(
            Rectangle::from_corners(Point2D::from_m(1., 2.), Point2D::from_m(1., 4.)),
            Sketch::empty()
        );
        assert_eq!(
            Rectangle::from_corners(Point2D::from_m(1., 2.), Point2D::from_m(3., 2.)),
            Sketch::empty()
        );
    }
}
