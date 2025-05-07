use cxx::UniquePtr;
use opencascade_sys::ffi;

use super::Point3D;

#[derive(Debug, PartialEq, Clone)]
pub struct Plane {
    origin: Point3D,
    normal: (f64, f64, f64),
}
impl Plane {
    pub fn xy() -> Self {
        Self {
            origin: Point3D::origin(),
            normal: (0., 0., 1.),
        }
    }

    pub(crate) fn to_occt_vec(&self) -> UniquePtr<ffi::gp_Vec> {
        ffi::new_vec(self.normal.0, self.normal.1, self.normal.2)
    }
}
