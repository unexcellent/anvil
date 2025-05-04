use crate::Length;

pub struct Point3D {
    pub x: Length,
    pub y: Length,
    pub z: Length,
}
impl Point3D {
    pub fn new(x: Length, y: Length, z: Length) -> Self {
        Point3D { x, y, z }
    }
    pub fn from_mm(x: f64, y: f64, z: f64) -> Self {
        Point3D {
            x: Length::from_mm(x),
            y: Length::from_mm(y),
            z: Length::from_mm(z),
        }
    }
    pub fn from_m(x: f64, y: f64, z: f64) -> Self {
        Point3D {
            x: Length::from_m(x),
            y: Length::from_m(y),
            z: Length::from_m(z),
        }
    }
}
