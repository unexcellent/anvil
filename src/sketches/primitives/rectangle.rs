use crate::{Length, Path, Point2D, Sketch};

/// Builder for a rectangular `Sketch`.
///
/// While the `Rectangle` struct itself is not used, its constructor methods like
/// `Rectangle::from_dim()` can be used to build this primitive `Sketch`.
pub struct Rectangle;
impl Rectangle {
    /// Construct a centered rectangular `Sketch` from the x and y dimensions.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Edge, Length, Rectangle, Point2D, Sketch};
    ///
    /// let sketch = Rectangle::from_dim(Length::from_m(2.), Length::from_m(6.));
    /// assert_eq!(
    ///     sketch.edges(),
    ///     &vec![
    ///         Edge::Line(Point2D::from_m(-1., -3.), Point2D::from_m(1., -3.)),
    ///         Edge::Line(Point2D::from_m(1., -3.), Point2D::from_m(1., 3.)),
    ///         Edge::Line(Point2D::from_m(1., 3.), Point2D::from_m(-1., 3.)),
    ///         Edge::Line(Point2D::from_m(-1., 3.), Point2D::from_m(-1., -3.)),
    ///     ]
    /// )
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

    /// Construct a centered rectangular `Sketch` from its corner locations.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Edge, Rectangle, Point2D, Sketch};
    ///
    /// let sketch = Rectangle::from_corners(Point2D::origin(), Point2D::from_m(2., 2.));
    /// assert_eq!(
    ///     sketch.edges(),
    ///     &vec![
    ///         Edge::Line(Point2D::origin(), Point2D::from_m(2., 0.)),
    ///         Edge::Line(Point2D::from_m(2., 0.), Point2D::from_m(2., 2.)),
    ///         Edge::Line(Point2D::from_m(2., 2.), Point2D::from_m(0., 2.)),
    ///         Edge::Line(Point2D::from_m(0., 2.), Point2D::origin()),
    ///     ]
    /// )
    /// ```
    pub fn from_corners(corner1: Point2D, corner2: Point2D) -> Sketch {
        Path::at(corner1)
            .line_to(Point2D::new(corner2.x, corner1.y))
            .line_to(corner2)
            .line_to(Point2D::new(corner1.x, corner2.y))
            .close()
    }
}
