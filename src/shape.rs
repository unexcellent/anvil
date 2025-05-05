use std::path::Path;

use cxx::UniquePtr;
use opencascade_sys::ffi;

use crate::{Error, Length, Point3D};

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
    pub fn move_to(&self, loc: Point3D) -> Self {
        match &self.inner {
            Some(inner) => {
                let mut transform = ffi::new_transform();
                transform.pin_mut().set_translation_vec(&ffi::new_vec(
                    loc.x.m(),
                    loc.y.m(),
                    loc.z.m(),
                ));
                let location = ffi::TopLoc_Location_from_transform(&transform);

                let mut new_inner = clone_topods_shape(inner);
                new_inner.pin_mut().set_global_translation(&location, false);

                Shape {
                    inner: Some(new_inner),
                }
            }
            None => Self { inner: None },
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

    pub fn write_step(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        match &self.inner {
            Some(inner) => {
                let mut writer = ffi::STEPControl_Writer_ctor();

                let status = ffi::transfer_shape(writer.pin_mut(), inner);

                if status != ffi::IFSelect_ReturnStatus::IFSelect_RetDone {
                    return Err(Error::StepWrite(path.as_ref().to_path_buf()));
                }

                let status = ffi::write_step(
                    writer.pin_mut(),
                    path.as_ref().to_string_lossy().to_string(),
                );

                if status != ffi::IFSelect_ReturnStatus::IFSelect_RetDone {
                    return Err(Error::StepWrite(path.as_ref().to_path_buf()));
                }
            }
            None => return Err(Error::StepWrite(path.as_ref().to_path_buf())),
        }
        Ok(())
    }
}

impl Clone for Shape {
    fn clone(&self) -> Self {
        match &self.inner {
            Some(inner) => Self::from_shape(inner),
            None => Shape { inner: None },
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

fn clone_topods_shape(inner: &UniquePtr<ffi::TopoDS_Shape>) -> UniquePtr<ffi::TopoDS_Shape> {
    ffi::TopoDS_Shape_to_owned(inner)
}

fn round(x: f64, n_digits: u8) -> f64 {
    (x * f64::from(10 ^ n_digits)).round() / f64::from(10 ^ n_digits)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::{Cuboid, Sphere};
    use tempdir::TempDir;

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
    fn eq_both_sphere() {
        let sphere1 = Sphere::from_radius(Length::from_m(2.));
        let sphere2 = Sphere::from_radius(Length::from_m(2.));
        assert!(sphere1 == sphere2)
    }

    #[test]
    fn neq_both_sphere() {
        let sphere1 = Sphere::from_radius(Length::from_m(1.));
        let sphere2 = Sphere::from_radius(Length::from_m(2.));
        assert!(sphere1 != sphere2)
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
    fn move_to() {
        let cuboid = Cuboid::from_m(1., 1., 1.);
        let loc = Point3D::from_m(2., 2., 2.);

        assert_eq!(cuboid.center_of_mass(), Some(Point3D::origin()));
        assert_eq!(cuboid.move_to(loc).center_of_mass(), Some(loc));
    }

    #[test]
    fn move_to_deepcopied() {
        let cuboid1 = Cuboid::from_m(1., 1., 1.);
        let loc = Point3D::from_m(2., 2., 2.);
        let cuboid2 = cuboid1.move_to(loc);

        assert_eq!(cuboid1.center_of_mass(), Some(Point3D::origin()));
        assert_eq!(cuboid2.center_of_mass(), Some(loc));
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

    #[test]
    fn write_step() {
        let tmp_dir = TempDir::new("step").expect("could not create tempdir");
        let path = tmp_dir.path().join("shape.step");
        let shape = Cuboid::from_m(1., 1., 1.);

        assert!(!path.is_file());
        shape.write_step(&path);
        assert!(path.is_file());
    }
}
