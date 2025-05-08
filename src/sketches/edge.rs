use cxx::UniquePtr;
use opencascade_sys::ffi;

use crate::{Length, Plane, Point2D, quantities::Axis};

/// A one-dimensional object in two-dimensional space.
#[derive(Debug, PartialEq, Clone)]
pub enum Edge {
    /// A circle at a center with a radius.
    Circle(Point2D, Length),

    /// A line between two points.
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
            Edge::Circle(center, _) => *center,
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
            Edge::Circle(center, _) => *center,
            Edge::Line(_, end) => *end,
        }
    }

    pub(crate) fn to_occt(&self, plane: &Plane) -> UniquePtr<ffi::TopoDS_Edge> {
        match self {
            Edge::Circle(center, radius) => {
                let axis = Axis {
                    origin: center.to_3d(plane),
                    direction: plane.normal(),
                };
                let circle = ffi::gp_Circ_ctor(&axis.to_occt(), radius.m());
                let mut constructor = ffi::BRepBuilderAPI_MakeEdge_circle(&circle);
                ffi::TopoDS_Edge_to_owned(constructor.pin_mut().Edge())
            }
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
