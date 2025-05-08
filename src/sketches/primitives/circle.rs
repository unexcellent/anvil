use crate::{Edge, Length, Point2D, Sketch};

/// Builder for a circular `Sketch`.
///
/// While the `Circle` struct itself is not used, its constructor methods like
/// `Circle::from_radius()` can be used to build this primitive `Sketch`.
pub struct Circle;
impl Circle {
    pub fn from_radius(radius: Length) -> Sketch {
        Sketch::from_edges(vec![Edge::Circle(Point2D::origin(), radius)])
    }
}
