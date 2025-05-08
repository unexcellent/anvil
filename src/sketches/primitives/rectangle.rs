use crate::{Length, Path, Point2D, Sketch};

/// Builder for a rectangular `Sketch`.
///
/// While the `Rectangle` struct itself is not used, its constructor methods like
/// `Rectangle::from_dim()` can be used to build this primitive `Sketch`.
pub struct Rectangle;
impl Rectangle {
    /// Construct a centered rectangular `Sketch` from the x and y dimensions.
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
    pub fn from_corners(corner1: Point2D, corner2: Point2D) -> Sketch {
        Path::at(corner1)
            .line_to(Point2D::new(corner2.x, corner1.y))
            .line_to(corner2)
            .line_to(Point2D::new(corner1.x, corner2.y))
            .close()
    }
}
