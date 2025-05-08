use cxx::UniquePtr;
use opencascade_sys::ffi;

use crate::{Length, Part, Plane};

use super::Edge;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Sketch(pub(crate) Vec<Edge>);
impl Sketch {
    /// Construct an empty `Sketch` which can be used for merging with other sketches.
    ///
    /// # Example
    /// ```rust
    /// use anvil::Sketch;
    ///
    /// let sketch = Sketch::empty();
    /// assert_eq!(sketch.area(), 0.);
    /// ```
    pub fn empty() -> Self {
        Self(vec![])
    }

    /// Return the edges that make up this `Sketch`.
    pub fn edges(&self) -> &Vec<Edge> {
        &self.0
    }

    /// Return true if this `Sketch` contains no edges.
    pub fn is_empty(&self) -> bool {
        self.edges().is_empty()
    }

    /// Return the area occupied by this `Sketch` in square meters.
    ///
    /// Warning: the area is susceptibility to floating point errors.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Rectangle, Length};
    ///
    /// let sketch = Rectangle::from_dim(Length::from_m(2.), Length::from_m(3.));
    /// assert!((sketch.area() - 6.).abs() < 1e-9)
    /// ```
    pub fn area(&self) -> f64 {
        if self.is_empty() {
            return 0.;
        }

        let mut gprops = ffi::GProp_GProps_ctor();
        ffi::BRepGProp_SurfaceProperties(
            ffi::cast_face_to_shape(&self.to_occt(&Plane::xy())),
            gprops.pin_mut(),
        );
        gprops.Mass()
    }

    /// Convert this `Sketch` into a `Part` by linearly extruding it.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Cuboid, Length, Rectangle, Plane, Point2D, Point3D};
    ///
    /// let sketch = Rectangle::from_corners(Point2D::origin(), Point2D::from_m(1., 2.));
    /// assert_eq!(
    ///     sketch.extrude(&Plane::xy(), Length::from_m(3.)),
    ///     Cuboid::from_corners(Point3D::origin(), Point3D::from_m(1., 2., 3.))
    /// );
    /// ```
    pub fn extrude(&self, plane: &Plane, thickness: Length) -> Part {
        let face = self.to_occt(plane);
        let face_shape = ffi::cast_face_to_shape(&face);
        let mut make_solid = ffi::BRepPrimAPI_MakePrism_ctor(
            face_shape,
            &(plane.normal() * thickness.m()).to_occt(),
            false,
            true,
        );

        Part::from_occt(make_solid.pin_mut().Shape())
    }

    pub(crate) fn to_occt(&self, plane: &Plane) -> UniquePtr<ffi::TopoDS_Face> {
        let occt_edges = self.edges().iter().map(|edge| edge.to_occt(plane));

        let mut make_wire = ffi::BRepBuilderAPI_MakeWire_ctor();
        for edge in occt_edges {
            make_wire.pin_mut().add_edge(&edge)
        }
        let wire = ffi::TopoDS_Wire_to_owned(make_wire.pin_mut().Wire());

        let make_face = ffi::BRepBuilderAPI_MakeFace_wire(&wire, false);
        let face = make_face.Face();
        ffi::TopoDS_Face_to_owned(face)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Cuboid, Path, Point2D, Point3D};

    use super::*;

    #[test]
    fn extrude_cube() {
        let sketch = Path::at(Point2D::origin())
            .line_to(Point2D::from_m(1., 0.))
            .line_to(Point2D::from_m(1., 2.))
            .line_to(Point2D::from_m(0., 2.))
            .close();
        assert_eq!(
            sketch.extrude(&Plane::xy(), Length::from_m(3.)),
            Cuboid::from_corners(Point3D::origin(), Point3D::from_m(1., 2., 3.))
        )
    }

    #[test]
    fn extrude_cube_different_plane() {
        let sketch = Path::at(Point2D::origin())
            .line_to(Point2D::from_m(1., 0.))
            .line_to(Point2D::from_m(1., 2.))
            .line_to(Point2D::from_m(0., 2.))
            .close();
        assert_eq!(
            sketch.extrude(&Plane::xz(), Length::from_m(-3.)),
            Cuboid::from_corners(Point3D::origin(), Point3D::from_m(1., 3., 2.))
        )
    }
}
