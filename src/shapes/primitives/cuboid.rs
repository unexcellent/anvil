use opencascade_sys::ffi;

use crate::{Length, Point3D, Shape};

/// Builder for a cuboidal `Shape`.
pub struct Cuboid;
impl Cuboid {
    /// Construct a centered cuboidal `Shape` from the x, y, and z dimensions.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Cuboid, Length, Point3D, Shape};
    ///
    /// let shape = Cuboid::from_dim(Length::from_m(1.), Length::from_m(2.), Length::from_m(3.));
    /// assert_eq!(shape.center_of_mass(), Some(Point3D::origin()));
    /// assert_eq!(shape.volume(), 6.);
    /// ```
    pub fn from_dim(x: Length, y: Length, z: Length) -> Shape {
        let corner1 = Point3D {
            x: x * -0.5,
            y: y * -0.5,
            z: z * -0.5,
        };
        let corner2 = Point3D {
            x: x * 0.5,
            y: y * 0.5,
            z: z * 0.5,
        };

        Self::from_corners(corner1, corner2)
    }
    /// Construct a centered cuboidal `Shape` from its corner locations.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Cuboid, Length, Point3D, Shape};
    ///
    /// let shape = Cuboid::from_corners(Point3D::origin(), Point3D::from_m(2., 2., 2.));
    /// assert_eq!(shape.center_of_mass(), Some(Point3D::from_m(1., 1., 1.)));
    /// assert_eq!(shape.volume(), 8.);
    /// ```
    pub fn from_corners(corner1: Point3D, corner2: Point3D) -> Shape {
        let min_x = corner1.x.min(&corner2.x).m();
        let min_y = corner1.y.min(&corner2.y).m();
        let min_z = corner1.z.min(&corner2.z).m();
        let max_x = corner1.x.max(&corner2.x).m();
        let max_y = corner1.y.max(&corner2.y).m();
        let max_z = corner1.z.max(&corner2.z).m();

        let point = ffi::new_point(min_x, min_y, min_z);
        let mut cuboid =
            ffi::BRepPrimAPI_MakeBox_ctor(&point, max_x - min_x, max_y - min_y, max_z - min_z);

        Shape::from_shape(cuboid.pin_mut().Shape())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Cuboid {
        pub fn from_m(x: f64, y: f64, z: f64) -> Shape {
            Cuboid::from_dim(Length::from_m(x), Length::from_m(y), Length::from_m(z))
        }
    }
}
