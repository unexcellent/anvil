use crate::{Length, Part, quantities::is_zero};
use opencascade_sys::ffi;

/// Builder for a spherical `Part`.
///
/// While the `Sphere` struct itself is not used, its constructor methods like `Sphere::from_radius()`
/// can be used to build this primitive `Part`.
pub struct Cylinder;
impl Cylinder {
    /// Construct a centered cylindrical `Part` from a given radius.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Cylinder, Length, Point3D, Part};
    ///
    /// let part = Cylinder::from_radius(Length::from_m(1.), Length::from_m(2.));
    /// assert_eq!(part.center_of_mass(), Ok(Point3D::origin()));
    /// assert!((part.volume() - 6.28319).abs() < 1e-5);
    /// ```
    pub fn from_radius(radius: Length, height: Length) -> Part {
        if is_zero(&[radius, height]) {
            return Part::empty();
        }
        let axis = ffi::gp_Ax2_ctor(
            &ffi::new_point(0., 0., -height.m() / 2.),
            &ffi::gp_Dir_ctor(0., 0., 1.),
        );
        let mut make = ffi::BRepPrimAPI_MakeCylinder_ctor(&axis, radius.m(), height.m());
        Part::from_part(make.pin_mut().Shape())
    }

    /// Construct a centered cylindrical `Part` from a given diameter.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Cylinder, Length, Point3D, Part};
    ///
    /// let part = Cylinder::from_diameter(Length::from_m(1.), Length::from_m(2.));
    /// assert_eq!(part.center_of_mass(), Ok(Point3D::origin()));
    /// assert!((part.volume() - 1.57080).abs() < 1e-5);
    /// ```
    pub fn from_diameter(diameter: Length, height: Length) -> Part {
        Self::from_radius(diameter / 2., height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_radius_empty() {
        assert!(Cylinder::from_radius(Length::from_m(0.), Length::from_m(1.)) == Part::empty());
        assert!(Cylinder::from_radius(Length::from_m(1.), Length::from_m(0.)) == Part::empty());
    }

    #[test]
    fn from_diameter_empty() {
        assert!(Cylinder::from_diameter(Length::from_m(0.), Length::from_m(1.)) == Part::empty());
        assert!(Cylinder::from_diameter(Length::from_m(1.), Length::from_m(0.)) == Part::empty());
    }
}
