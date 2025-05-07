use cxx::UniquePtr;
use opencascade_sys::ffi;

use crate::{Plane, Point2D};

#[derive(Debug, PartialEq, Clone)]
pub enum Edge {
    Line(Point2D, Point2D),
}
impl Edge {
    pub(crate) fn to_occt(&self, plane: &Plane) -> UniquePtr<ffi::TopoDS_Edge> {
        match self {
            Edge::Line(start, end) => {
                let mut constructor = ffi::BRepBuilderAPI_MakeEdge_gp_Pnt_gp_Pnt(
                    &start.to_3d(plane).to_occt_point(),
                    &end.to_3d(plane).to_occt_point(),
                );
                ffi::TopoDS_Edge_to_owned(constructor.pin_mut().Edge())
            }
        }
    }
}
