use std::ops::Mul;

use cxx::UniquePtr;
use opencascade_sys::ffi;

use crate::Error;

use super::{Length, Point3D};

/// A normalized direction in 3D space.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Dir3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Dir3 {
    /// Construct a `Dir3` from the directional components.
    pub fn from(vec: (f64, f64, f64)) -> Self {
        Dir3 {
            x: vec.0,
            y: vec.1,
            z: vec.2,
        }
    }

    /// Return the absolute length of this `Dir3`.
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Return a `Dir3` that has the same direction as this one but a magnitude of 1.
    pub fn normalize(&self) -> Result<Self, Error> {
        let mag = self.magnitude();
        if mag == 0. {
            Err(Error::ZeroVector(*self))
        } else {
            Ok(Self {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            })
        }
    }

    /// Return the dot-product of this `Dir3` with another.
    pub fn dot(&self, other: Dir3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    /// Return the cross-product of this `Dir3` with another.
    pub fn cross(&self, other: Dir3) -> Dir3 {
        Dir3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub(crate) fn to_occt_vec(self) -> UniquePtr<ffi::gp_Vec> {
        ffi::new_vec(self.x, self.y, self.z)
    }

    pub(crate) fn to_occt_dir(self) -> UniquePtr<ffi::gp_Dir> {
        ffi::gp_Dir_ctor(self.x, self.y, self.z)
    }
}

impl Mul<Length> for Dir3 {
    type Output = Point3D;
    fn mul(self, other: Length) -> Point3D {
        Point3D {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Dir3> for Length {
    type Output = Point3D;
    fn mul(self, other: Dir3) -> Point3D {
        other * self
    }
}

impl Mul<f64> for Dir3 {
    type Output = Dir3;
    fn mul(self, other: f64) -> Dir3 {
        Dir3::from((self.x * other, self.y * other, self.z * other))
    }
}

impl Mul<Dir3> for f64 {
    type Output = Dir3;
    fn mul(self, other: Dir3) -> Dir3 {
        other * self
    }
}
