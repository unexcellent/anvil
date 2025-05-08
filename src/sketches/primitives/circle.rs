use crate::{Edge, Length, Point2D, Sketch};

/// Builder for a circular `Sketch`.
///
/// While the `Circle` struct itself is not used, its constructor methods like
/// `Circle::from_radius()` can be used to build this primitive `Sketch`.
pub struct Circle;
impl Circle {
    /// Construct a centered circular `Sketch` from a given radius.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Circle, Length, Point2D};
    ///
    /// let circle = Circle::from_radius(Length::from_m(1.));
    /// assert!((circle.area() - 3.141593).abs() < 1e-5);
    /// assert_eq!(circle.center(), Ok(Point2D::origin()));
    /// ```
    pub fn from_radius(radius: Length) -> Sketch {
        Sketch::from_edges(vec![Edge::Circle(Point2D::origin(), radius)])
    }

    /// Construct a centered circular `Sketch` from a given diameter.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Circle, Length, Point2D};
    ///
    /// let circle = Circle::from_diameter(Length::from_m(1.));
    /// assert!((circle.area() - 0.785398).abs() < 1e-5);
    /// assert_eq!(circle.center(), Ok(Point2D::origin()));
    /// ```
    pub fn from_diameter(diameter: Length) -> Sketch {
        Sketch::from_edges(vec![Edge::Circle(Point2D::origin(), diameter / 2.)])
    }
}
