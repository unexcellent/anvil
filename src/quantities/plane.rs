use super::Point3D;

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
}
