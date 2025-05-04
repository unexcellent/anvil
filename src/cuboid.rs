use opencascade_sys::ffi;

use crate::{Length, Point3D, Shape};

pub struct Cuboid;
impl Cuboid {
    pub fn from_corners(corner1: Point3D, corner2: Point3D) -> Shape {
        let min_x = if corner1.x.m() < corner2.x.m() {
            corner1.x.m()
        } else {
            corner2.x.m()
        };
        let min_y = if corner1.y.m() < corner2.y.m() {
            corner1.y.m()
        } else {
            corner2.y.m()
        };
        let min_z = if corner1.z.m() < corner2.z.m() {
            corner1.z.m()
        } else {
            corner2.z.m()
        };
        let max_x = if corner1.x.m() >= corner2.x.m() {
            corner1.x.m()
        } else {
            corner2.x.m()
        };
        let max_y = if corner1.y.m() >= corner2.y.m() {
            corner1.y.m()
        } else {
            corner2.y.m()
        };
        let max_z = if corner1.z.m() >= corner2.z.m() {
            corner1.z.m()
        } else {
            corner2.z.m()
        };

        let point = ffi::new_point(min_x, min_y, min_z);
        let mut cuboid =
            ffi::BRepPrimAPI_MakeBox_ctor(&point, max_x - min_x, max_y - min_y, max_z - min_z);

        Shape::from_shape(cuboid.pin_mut().Shape())
    }
}
