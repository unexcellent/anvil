use cxx::UniquePtr;
use opencascade_sys::ffi;

use crate::{Length, Point2D, Point3D};

#[derive(Debug, PartialEq, Clone)]
pub enum Edge {
    Line(Point2D, Point2D),
}
impl Edge {
    pub(crate) fn to_occt(&self) -> UniquePtr<ffi::TopoDS_Edge> {
        match self {
            Edge::Line(start, end) => {
                let p1 = Point3D::new(start.x, start.y, Length::zero());
                let p2 = Point3D::new(end.x, end.y, Length::zero());

                let mut constructor = ffi::BRepBuilderAPI_MakeEdge_gp_Pnt_gp_Pnt(
                    &p1.to_occt_point(),
                    &p2.to_occt_point(),
                );
                ffi::TopoDS_Edge_to_owned(constructor.pin_mut().Edge())
            }
        }
    }
}
