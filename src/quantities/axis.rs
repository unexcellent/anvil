use cxx::UniquePtr;
use opencascade_sys::ffi;

use super::{Point3D, Vec3};

pub struct Axis {
    pub origin: Point3D,
    pub direction: Vec3,
}
impl Axis {
    pub fn new(origin: Point3D, direction: (f64, f64, f64)) -> Self {
        Self {
            origin,
            direction: Vec3::from(direction),
        }
    }

    pub(crate) fn to_occt(&self) -> UniquePtr<ffi::gp_Ax2> {
        ffi::gp_Ax2_ctor(&self.origin.to_occt_point(), &self.direction.to_occt_dir())
    }
}
