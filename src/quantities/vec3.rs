use std::ops::Mul;

use cxx::UniquePtr;
use opencascade_sys::ffi;

use crate::Error;

use super::{Length, Point3D};

/// A unitless vector in 3D space.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vec3 {
    /// Construct a `Vec3` from the directional components.
    pub fn from(vec: (f64, f64, f64)) -> Self {
        Vec3 {
            x: vec.0,
            y: vec.1,
            z: vec.2,
        }
    }

    /// Return the absolute length of this `Vec3`.
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Return a `Vec3` that has the same direction as this one but a magnitude of 1.
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

    /// Return the dot-product of this `Vec3` with another.
    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    /// Return the cross-product of this `Vec3` with another.
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
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

impl Mul<Length> for Vec3 {
    type Output = Point3D;
    fn mul(self, other: Length) -> Point3D {
        Point3D {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vec3> for Length {
    type Output = Point3D;
    fn mul(self, other: Vec3) -> Point3D {
        other * self
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f64) -> Vec3 {
        Vec3::from((self.x * other, self.y * other, self.z * other))
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}
