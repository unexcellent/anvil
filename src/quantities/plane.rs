use cxx::UniquePtr;
use opencascade_sys::ffi;

use crate::Error;

use super::{Length, Point3D, vec3::Vec3};

#[derive(Debug, PartialEq, Clone)]
pub struct Plane {
    origin: Point3D,
    normal: Vec3,
    x_direction: Vec3,
}
impl Plane {
    pub fn xy() -> Self {
        Self {
            origin: Point3D::origin(),
            normal: Vec3::from((0., 0., 1.)),
            x_direction: Vec3::from((1., 0., 0.)),
        }
    }
    pub fn xz() -> Self {
        Self {
            origin: Point3D::origin(),
            normal: Vec3::from((0., 1., 0.)),
            x_direction: Vec3::from((1., 0., 0.)),
        }
    }
    pub fn yz() -> Self {
        Self {
            origin: Point3D::origin(),
            normal: Vec3::from((1., 0., 0.)),
            x_direction: Vec3::from((0., 1., 0.)),
        }
    }
    pub fn new(
        origin: Point3D,
        normal: (f64, f64, f64),
        x_direction: (f64, f64, f64),
    ) -> Result<Self, Error> {
        Ok(Self {
            origin,
            normal: Vec3::from(normal).normalize()?,
            x_direction: Vec3::from(x_direction).normalize()?,
        })
    }

    pub fn origin(&self) -> Point3D {
        self.origin
    }
    pub fn normal(&self) -> Vec3 {
        self.normal
    }
    pub fn basis(&self) -> (Vec3, Vec3) {
        (self.x_direction, self.normal.cross(self.x_direction))
    }

    pub(crate) fn normal_to_occt_vec(&self, mag: Length) -> UniquePtr<ffi::gp_Vec> {
        ffi::new_vec(
            self.normal.x * mag.m(),
            self.normal.y * mag.m(),
            self.normal.z * mag.m(),
        )
    }
}
