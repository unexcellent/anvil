use opencascade_sys::ffi;

use crate::{Length, Part, Plane, Point2D};

use super::Edge;

pub struct Sketch {
    plane: Plane,
    cursor: Point2D,
    edges: Vec<Edge>,
}
impl Sketch {
    pub fn new(plane: Plane) -> Self {
        Self {
            plane,
            cursor: Point2D::origin(),
            edges: vec![],
        }
    }
    pub fn line_to(&self, point: Point2D) -> Self {
        let mut new_edges = self.edges.clone();
        new_edges.push(Edge::Line(self.cursor, point));

        Self {
            plane: self.plane.clone(),
            cursor: point,
            edges: new_edges,
        }
    }
    pub fn extrude(&self, thickness: Length) -> Part {
        let occt_edges = self.edges.iter().map(|edge| edge.to_occt(&self.plane));

        let mut make_wire = ffi::BRepBuilderAPI_MakeWire_ctor();
        for edge in occt_edges {
            make_wire.pin_mut().add_edge(&edge)
        }
        let wire = ffi::TopoDS_Wire_to_owned(make_wire.pin_mut().Wire());

        let make_face = ffi::BRepBuilderAPI_MakeFace_wire(&wire, false);
        let face = make_face.Face();

        let face_shape = ffi::cast_face_to_shape(face);
        let mut make_solid = ffi::BRepPrimAPI_MakePrism_ctor(
            face_shape,
            &(self.plane.normal() * thickness.m()).to_occt(),
            false,
            true,
        );

        Part::from_occt(make_solid.pin_mut().Shape())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Cuboid, Point3D};

    use super::*;

    #[test]
    fn line_to() {
        let sketch = Sketch::new(Plane::xy()).line_to(Point2D::from_m(1., 2.));
        assert_eq!(
            sketch.edges,
            vec![Edge::Line(Point2D::origin(), Point2D::from_m(1., 2.))]
        );
        assert_eq!(sketch.cursor, Point2D::from_m(1., 2.));
    }

    #[test]
    fn extrude_cube() {
        let sketch = Sketch::new(Plane::xy())
            .line_to(Point2D::from_m(1., 0.))
            .line_to(Point2D::from_m(1., 2.))
            .line_to(Point2D::from_m(0., 2.))
            .line_to(Point2D::origin());
        assert_eq!(
            sketch.extrude(Length::from_m(3.)),
            Cuboid::from_corners(Point3D::origin(), Point3D::from_m(1., 2., 3.))
        )
    }

    #[test]
    fn extrude_cube_different_plane() {
        let sketch = Sketch::new(Plane::xz())
            .line_to(Point2D::from_m(1., 0.))
            .line_to(Point2D::from_m(1., 2.))
            .line_to(Point2D::from_m(0., 2.))
            .line_to(Point2D::origin());
        assert_eq!(
            sketch.extrude(Length::from_m(-3.)),
            Cuboid::from_corners(Point3D::origin(), Point3D::from_m(1., 3., 2.))
        )
    }
}
