use crate::{Length, Shape};
use opencascade_sys::ffi;

pub struct Sphere;
impl Sphere {
    pub fn from_radius(radius: Length) -> Shape {
        let mut make_sphere = ffi::BRepPrimAPI_MakeSphere_ctor(radius.m());
        Shape::from_shape(make_sphere.pin_mut().Shape())
    }
    pub fn from_diameter(diameter: Length) -> Shape {
        Self::from_radius(diameter * 0.5)
    }
}
