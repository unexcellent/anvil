use crate::{Length, Part, quantities::is_zero};
use opencascade_sys::ffi;

/// Builder for a cylindrical `Part`.
///
/// While the `Cylinder` struct itself is not used, its constructor methods like
/// `Cylinder::from_radius()` can be used to build this primitive `Part`.
#[derive(Debug, PartialEq, Clone)]
pub struct Cylinder;
impl Cylinder {
    /// Construct a centered cylindrical `Part` from a given radius.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Cylinder, length, Point3D, Part};
    ///
    /// let part = Cylinder::from_radius(length!(1 m), length!(2 m));
    /// assert_eq!(part.center(), Ok(Point3D::origin()));
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
        Part::from_occt(make.pin_mut().Shape())
    }

    /// Construct a centered cylindrical `Part` from a given diameter.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Cylinder, length, Point3D, Part};
    ///
    /// let part = Cylinder::from_diameter(length!(1 m), length!(2 m));
    /// assert_eq!(part.center(), Ok(Point3D::origin()));
    /// assert!((part.volume() - 1.57080).abs() < 1e-5);
    /// ```
    pub fn from_diameter(diameter: Length, height: Length) -> Part {
        Self::from_radius(diameter / 2., height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::length;

    #[test]
    fn from_radius_empty() {
        assert!(Cylinder::from_radius(length!(0), length!(1 m)) == Part::empty());
        assert!(Cylinder::from_radius(length!(1 m), length!(0)) == Part::empty());
    }

    #[test]
    fn from_diameter_empty() {
        assert!(Cylinder::from_diameter(length!(0), length!(1 m)) == Part::empty());
        assert!(Cylinder::from_diameter(length!(1 m), length!(0)) == Part::empty());
    }
}
