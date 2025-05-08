use cxx::UniquePtr;
use opencascade_sys::ffi;

use crate::{Plane, Point2D};

/// A one-dimensional object in two-dimensional space.
#[derive(Debug, PartialEq, Clone)]
pub enum Edge {
    Line(Point2D, Point2D),
}
impl Edge {
    /// Return the starting point of the edge.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Edge, Point2D};
    ///
    /// let edge = Edge::Line(Point2D::from_m(1., 1.), Point2D::from_m(2., 2.));
    /// assert_eq!(edge.start(), Point2D::from_m(1., 1.))
    /// ```
    pub fn start(&self) -> Point2D {
        match self {
            Edge::Line(start, _) => *start,
        }
    }
    /// Return the ending point of the edge.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Edge, Point2D};
    ///
    /// let edge = Edge::Line(Point2D::from_m(1., 1.), Point2D::from_m(2., 2.));
    /// assert_eq!(edge.end(), Point2D::from_m(2., 2.))
    /// ```
    pub fn end(&self) -> Point2D {
        match self {
            Edge::Line(_, end) => *end,
        }
    }

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
