use crate::{Length, Shape, quantities::is_zero};
use opencascade_sys::ffi;

/// Builder for a spherical `Shape`.
///
/// While the `Sphere` struct itself is not used, its constructor methods like `Sphere::from_radius()`
/// can be used to build this primitive `Shape`.
pub struct Sphere;
impl Sphere {
    /// Construct a centered spherical `Shape` from a given radius.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Sphere, Length, Point3D, Shape};
    ///
    /// let shape = Sphere::from_radius(Length::from_m(1.));
    /// assert_eq!(shape.center_of_mass(), Some(Point3D::origin()));
    /// assert!((shape.volume() - 4.18879).abs() < 1e-5);
    /// ```
    pub fn from_radius(radius: Length) -> Shape {
        if is_zero(&[radius]) {
            return Shape::empty();
        }
        let mut make_sphere = ffi::BRepPrimAPI_MakeSphere_ctor(radius.m());
        Shape::from_shape(make_sphere.pin_mut().Shape())
    }
    /// Construct a centered spherical `Shape` from a given diameter.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Sphere, Length, Point3D, Shape};
    ///
    /// let shape = Sphere::from_diameter(Length::from_m(1.));
    /// assert_eq!(shape.center_of_mass(), Some(Point3D::origin()));
    /// assert!((shape.volume() - 0.523599).abs() < 1e-5);
    /// ```
    pub fn from_diameter(diameter: Length) -> Shape {
        Self::from_radius(diameter / 2.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_radius_empty() {
        assert!(Sphere::from_radius(Length::from_m(0.)) == Shape::empty())
    }

    #[test]
    fn from_diameter_empty() {
        assert!(Sphere::from_diameter(Length::from_m(0.)) == Shape::empty())
    }
}
