use cxx::UniquePtr;
use opencascade_sys::ffi;

use crate::{Length, Point3D};

pub struct Shape {
    pub(crate) inner: Option<UniquePtr<ffi::TopoDS_Shape>>,
}
impl Shape {
    pub(crate) fn from_shape(shape: &ffi::TopoDS_Shape) -> Self {
        let inner = ffi::TopoDS_Shape_to_owned(shape);
        Self { inner: Some(inner) }
    }
    pub fn empty() -> Self {
        Self { inner: None }
    }

    pub fn intersect(&self, other: &Self) -> Self {
        match (&self.inner, &other.inner) {
            (Some(self_inner), Some(other_inner)) => {
                let mut fuse_operation = ffi::BRepAlgoAPI_Common_ctor(self_inner, other_inner);
                Self::from_shape(fuse_operation.pin_mut().Shape())
            }
            _ => Shape { inner: None },
        }
    }

    pub fn volume(&self) -> f64 {
        match &self.inner {
            Some(inner) => {
                let mut gprops = ffi::GProp_GProps_ctor();
                ffi::BRepGProp_VolumeProperties(inner, gprops.pin_mut());
                round(gprops.Mass(), 9)
            }
            None => 0.,
        }
    }
    pub fn center_of_mass(&self) -> Option<Point3D> {
        match &self.inner {
            Some(inner) => {
                let mut gprops = ffi::GProp_GProps_ctor();
                ffi::BRepGProp_VolumeProperties(inner, gprops.pin_mut());
                let centre_of_mass = ffi::GProp_GProps_CentreOfMass(&gprops);

                Some(Point3D {
                    x: Length::from_m(round(centre_of_mass.X(), 9)),
                    y: Length::from_m(round(centre_of_mass.Y(), 9)),
                    z: Length::from_m(round(centre_of_mass.Z(), 9)),
                })
            }
            None => None,
        }
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        match (&self.inner, &other.inner) {
            (Some(_), Some(_)) => {
                let intersection = self.intersect(other);

                (intersection.volume() - self.volume()).abs() < 1e-7
                    && (intersection.volume() - other.volume()).abs() < 1e-7
            }
            (Some(_), None) => false,
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }
}

fn round(x: f64, n_digits: u8) -> f64 {
    (x * f64::from(10 ^ n_digits)).round() / f64::from(10 ^ n_digits)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Cuboid;

    #[test]
    fn eq_both_none() {
        assert!(Shape::empty() == Shape::empty())
    }

    #[test]
    fn eq_both_cuboid() {
        let cuboid1 = Cuboid::from_m(1., 1., 1.);
        let cuboid2 = Cuboid::from_m(1., 1., 1.);
        assert!(cuboid1 == cuboid2)
    }

    #[test]
    fn neq_both_cuboid() {
        let cuboid1 = Cuboid::from_m(1., 1., 1.);
        let cuboid2 = Cuboid::from_m(2., 2., 2.);
        assert!(cuboid1 != cuboid2)
    }

    #[test]
    fn intersect_same_shape() {
        let cuboid1 = Cuboid::from_m(1., 1., 1.);
        let cuboid2 = Cuboid::from_m(1., 1., 1.);

        assert!(cuboid1.intersect(&cuboid2) == cuboid1);
        assert!(cuboid2.intersect(&cuboid1) == cuboid2);
    }

    #[test]
    fn intersect_different_shape() {
        let cuboid1 = Cuboid::from_m(5., 5., 1.);
        let cuboid2 = Cuboid::from_m(1., 1., 5.);
        let right = Cuboid::from_m(1., 1., 1.);

        assert!(cuboid1.intersect(&cuboid2) == right);
        assert!(cuboid2.intersect(&cuboid1) == right);
    }

    #[test]
    fn volume() {
        let cuboid = Cuboid::from_m(1., 1., 1.);
        assert_eq!(cuboid.volume(), 1.)
    }

    #[test]
    fn centre_of_mass_at_origin() {
        let cuboid = Cuboid::from_m(1., 1., 1.);
        assert_eq!(cuboid.center_of_mass(), Some(Point3D::from_m(0., 0., 0.)))
    }

    #[test]
    fn centre_of_mass_not_at_origin() {
        let cuboid = Cuboid::from_corners(Point3D::from_m(0., 0., 0.), Point3D::from_m(2., 2., 2.));
        assert_eq!(cuboid.center_of_mass(), Some(Point3D::from_m(1., 1., 1.)))
    }
}
