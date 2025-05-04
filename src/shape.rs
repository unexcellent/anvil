use cxx::UniquePtr;
use opencascade_sys::ffi;

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
                gprops.Mass()
            }
            None => 0.,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Cuboid, Length};

    fn round(x: f64, n_digits: u8) -> f64 {
        (x * f64::from(10 ^ n_digits)).round() / f64::from(10 ^ n_digits)
    }

    #[test]
    fn eq_both_none() {
        assert!(Shape::empty() == Shape::empty())
    }

    #[test]
    fn eq_both_cuboid() {
        let cuboid1 = Cuboid::from_dim(Length::from_m(1.), Length::from_m(1.), Length::from_m(1.));
        let cuboid2 = Cuboid::from_dim(Length::from_m(1.), Length::from_m(1.), Length::from_m(1.));
        assert!(cuboid1 == cuboid2)
    }

    #[test]
    fn neq_both_cuboid() {
        let cuboid1 = Cuboid::from_dim(Length::from_m(1.), Length::from_m(1.), Length::from_m(1.));
        let cuboid2 = Cuboid::from_dim(Length::from_m(2.), Length::from_m(2.), Length::from_m(2.));
        assert!(cuboid1 != cuboid2)
    }

    #[test]
    fn volume() {
        let cuboid = Cuboid::from_dim(Length::from_m(1.), Length::from_m(1.), Length::from_m(1.));
        assert_eq!(round(cuboid.volume(), 6), 1.)
    }
}
