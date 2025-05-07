use crate::Error;

use super::{Point3D, vec3::Vec3};

#[derive(Debug, PartialEq, Clone)]
pub struct Plane {
    origin: Point3D,
    x_axis: Vec3,
    y_axis: Vec3,
}
impl Plane {
    pub fn xy() -> Self {
        Self::new(Point3D::origin(), (1., 0., 0.), (0., 1., 0.)).expect("error in axis def")
    }
    pub fn xz() -> Self {
        Self::new(Point3D::origin(), (1., 0., 0.), (0., 0., 1.)).expect("error in axis def")
    }
    pub fn yz() -> Self {
        Self::new(Point3D::origin(), (0., 1., 0.), (0., 0., 1.)).expect("error in axis def")
    }
    pub fn new(
        origin: Point3D,
        x_axis: (f64, f64, f64),
        y_axis: (f64, f64, f64),
    ) -> Result<Self, Error> {
        Ok(Self {
            origin,
            x_axis: Vec3::from(x_axis).normalize()?,
            y_axis: Vec3::from(y_axis).normalize()?,
        })
    }

    pub fn origin(&self) -> Point3D {
        self.origin
    }
    pub fn normal(&self) -> Vec3 {
        self.y_axis.cross(self.x_axis)
    }
    pub fn x_axis(&self) -> Vec3 {
        self.x_axis
    }
    pub fn y_axis(&self) -> Vec3 {
        self.y_axis
    }
}
