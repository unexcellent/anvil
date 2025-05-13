use std::vec;

use cxx::UniquePtr;
use opencascade_sys::ffi;

use crate::{Angle, Axis, Error, Length, Part, Plane, Point2D};

use super::Edge;

/// A closed shape in 2D space.
#[derive(Debug, Clone)]
pub struct Sketch(Vec<SketchAction>);
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

    /// Return true if this `Sketch` is empty.
    pub fn is_empty(&self) -> bool {
        self.to_occt(&Plane::xy()).is_err()
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
        match self.to_occt(&Plane::xy()) {
            Ok(occt) => occt_area(&occt),
            Err(_) => 0.,
        }
    }
    /// Return the center of mass of the `Sketch`.
    ///
    /// If the `Sketch` is empty, an `Err(Error::EmptySketch)` is returned.
    ///
    /// # Examples
    /// ```rust
    /// use anvil::{Error, Length, Point2D, Rectangle, Sketch};
    ///
    /// let centered_rect = Rectangle::from_dim(Length::from_m(1.), Length::from_m(2.));
    /// let moved_rect = centered_rect.move_to(Point2D::from_m(3., 3.));
    /// assert_eq!(centered_rect.center(), Ok(Point2D::origin()));
    /// assert_eq!(moved_rect.center(), Ok(Point2D::from_m(3., 3.)));
    /// assert_eq!(Sketch::empty().center(), Err(Error::EmptySketch));
    /// ```
    pub fn center(&self) -> Result<Point2D, Error> {
        let occt = self.to_occt(&Plane::xy())?;
        let mut gprops = ffi::GProp_GProps_ctor();
        ffi::BRepGProp_VolumeProperties(&occt, gprops.pin_mut());

        let centre_of_mass = ffi::GProp_GProps_CentreOfMass(&gprops);
        Ok(Point2D::from_m(centre_of_mass.X(), centre_of_mass.Y()))
    }

    /// Merge this `Sketch` with another.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Rectangle, Point2D};
    ///
    /// let sketch1 = Rectangle::from_corners(Point2D::origin(), Point2D::from_m(1., 2.));
    /// let sketch2 = Rectangle::from_corners(Point2D::from_m(1., 0.), Point2D::from_m(2., 2.));
    /// assert_eq!(
    ///     sketch1.add(&sketch2),
    ///     Rectangle::from_corners(Point2D::origin(), Point2D::from_m(2., 2.))
    /// )
    /// ```
    pub fn add(&self, other: &Self) -> Self {
        let mut new_actions = self.0.clone();
        new_actions.push(SketchAction::Add(other.clone()));
        Self(new_actions)
    }
    /// Return the `Sketch` that is created from the overlapping area between this one and another.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Rectangle, Point2D};
    ///
    /// let sketch1 = Rectangle::from_corners(Point2D::origin(), Point2D::from_m(2., 2.));
    /// let sketch2 = Rectangle::from_corners(Point2D::origin(), Point2D::from_m(1., 2.));
    /// assert_eq!(
    ///     sketch1.intersect(&sketch2),
    ///     Rectangle::from_corners(Point2D::origin(), Point2D::from_m(1., 2.))
    /// )
    /// ```
    pub fn intersect(&self, other: &Self) -> Self {
        let mut new_actions = self.0.clone();
        new_actions.push(SketchAction::Intersect(other.clone()));
        Self(new_actions)
    }
    /// Return a copy of this `Sketch` with the intersection of another removed.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Rectangle, Point2D};
    ///
    /// let sketch1 = Rectangle::from_corners(Point2D::origin(), Point2D::from_m(2., 2.));
    /// let sketch2 = Rectangle::from_corners(Point2D::from_m(1., 0.), Point2D::from_m(2., 2.));
    /// assert_eq!(
    ///     sketch1.subtract(&sketch2),
    ///     Rectangle::from_corners(Point2D::origin(), Point2D::from_m(1., 2.))
    /// )
    /// ```
    pub fn subtract(&self, other: &Self) -> Self {
        let mut new_actions = self.0.clone();
        new_actions.push(SketchAction::Subtract(other.clone()));
        Self(new_actions)
    }
    /// Return a clone of this `Sketch` with the center moved to a specified point.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Length, Point2D, Rectangle};
    ///
    /// let rect = Rectangle::from_dim(Length::from_m(1.), Length::from_m(1.));
    /// let moved_rect = rect.move_to(Point2D::from_m(2., 2.));
    /// assert_eq!(rect.center(), Ok(Point2D::origin()));
    /// assert_eq!(moved_rect.center(), Ok(Point2D::from_m(2., 2.)));
    /// ```
    pub fn move_to(&self, loc: Point2D) -> Self {
        let mut new_actions = self.0.clone();
        new_actions.push(SketchAction::MoveTo(loc));
        Self(new_actions)
    }
    /// Return a clone of this `Sketch` rotated around its center.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Angle, Length, Point2D, Rectangle};
    ///
    /// let sketch = Rectangle::from_dim(Length::from_m(1.), Length::from_m(2.)).move_to(Point2D::from_m(1, 1));
    /// assert_eq!(
    ///     sketch.rotate(Angle::from_deg(90.)),
    ///     Rectangle::from_dim(Length::from_m(2.), Length::from_m(1.)).move_to(Point2D::from_m(1, 1))
    /// )
    /// ```
    pub fn rotate(&self, angle: Angle) -> Self {
        let mut new_actions = self.0.clone();
        new_actions.push(SketchAction::Rotate(angle));
        Self(new_actions)
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
    ///     Ok(Cuboid::from_corners(Point3D::origin(), Point3D::from_m(1., 2., 3.)))
    /// );
    /// ```
    pub fn extrude(&self, plane: &Plane, thickness: Length) -> Result<Part, Error> {
        if thickness == Length::zero() {
            return Err(Error::EmptySketch);
        }

        let shape = self.to_occt(plane)?;
        let mut make_solid = ffi::BRepPrimAPI_MakePrism_ctor(
            &shape,
            &(plane.normal() * thickness.m()).to_occt_vec(),
            false,
            true,
        );

        Ok(Part::from_occt(make_solid.pin_mut().Shape()))
    }

    pub(crate) fn from_edges(edges: Vec<Edge>) -> Self {
        Self(vec![SketchAction::AddEdges(edges)])
    }

    pub(crate) fn to_occt(&self, plane: &Plane) -> Result<UniquePtr<ffi::TopoDS_Shape>, Error> {
        let mut occt = None;
        for action in &self.0 {
            occt = action.apply(occt, plane);
        }

        match occt {
            Some(face) => Ok(face),
            None => Err(Error::EmptySketch),
        }
    }
}

impl PartialEq for Sketch {
    fn eq(&self, other: &Self) -> bool {
        if self.center() != other.center() {
            return false;
        }

        match self.intersect(other).to_occt(&Plane::xy()) {
            Ok(intersection) => {
                (occt_area(&intersection) - self.area()).abs() < 1e-7
                    && (occt_area(&intersection) - other.area()).abs() < 1e-7
            }
            Err(_) => true,
        }
    }
}

fn edges_to_occt(edges: &[Edge], plane: &Plane) -> Result<UniquePtr<ffi::TopoDS_Shape>, Error> {
    let occt_edges: Vec<UniquePtr<ffi::TopoDS_Edge>> = edges
        .iter()
        .filter_map(|edge| edge.to_occt(plane))
        .collect();

    if occt_edges.is_empty() {
        return Err(Error::EmptySketch);
    }

    let mut make_wire = ffi::BRepBuilderAPI_MakeWire_ctor();
    for edge in occt_edges {
        make_wire.pin_mut().add_edge(&edge)
    }
    let wire = ffi::TopoDS_Wire_to_owned(make_wire.pin_mut().Wire());

    let make_face = ffi::BRepBuilderAPI_MakeFace_wire(&wire, false);
    let face = make_face.Face();
    Ok(ffi::TopoDS_Shape_to_owned(ffi::cast_face_to_shape(face)))
}

fn occt_area(occt: &ffi::TopoDS_Shape) -> f64 {
    let mut gprops = ffi::GProp_GProps_ctor();
    ffi::BRepGProp_SurfaceProperties(occt, gprops.pin_mut());
    gprops.Mass()
}

#[derive(Debug, PartialEq, Clone)]
enum SketchAction {
    Add(Sketch),
    AddEdges(Vec<Edge>),
    Intersect(Sketch),
    MoveTo(Point2D),
    Rotate(Angle),
    Subtract(Sketch),
}
impl SketchAction {
    pub fn apply(
        &self,
        sketch: Option<UniquePtr<ffi::TopoDS_Shape>>,
        plane: &Plane,
    ) -> Option<UniquePtr<ffi::TopoDS_Shape>> {
        match self {
            SketchAction::Add(other) => match (sketch, other.to_occt(plane).ok()) {
                (None, None) => None,
                (None, Some(other)) => Some(other),
                (Some(sketch), None) => Some(sketch),
                (Some(self_shape), Some(other_shape)) => {
                    let mut operation = ffi::BRepAlgoAPI_Fuse_ctor(&self_shape, &other_shape);
                    Some(ffi::TopoDS_Shape_to_owned(operation.pin_mut().Shape()))
                }
            },
            SketchAction::AddEdges(edges) => match sketch {
                None => edges_to_occt(edges, plane).ok(),
                Some(_) => todo!(),
            },
            SketchAction::Intersect(other) => match (sketch, other.to_occt(plane).ok()) {
                (Some(self_shape), Some(other_shape)) => {
                    let mut operation = ffi::BRepAlgoAPI_Common_ctor(&self_shape, &other_shape);
                    let new_shape = ffi::TopoDS_Shape_to_owned(operation.pin_mut().Shape());
                    if occt_area(&new_shape) == 0. {
                        None
                    } else {
                        Some(new_shape)
                    }
                }
                _ => None,
            },
            SketchAction::MoveTo(loc) => match sketch {
                Some(shape) => {
                    let mut transform = ffi::new_transform();
                    transform
                        .pin_mut()
                        .set_translation_vec(&loc.to_3d(plane).to_occt_vec());
                    let location = ffi::TopLoc_Location_from_transform(&transform);

                    let mut new_inner = ffi::TopoDS_Shape_to_owned(&shape);
                    new_inner.pin_mut().set_global_translation(&location, false);

                    Some(new_inner)
                }
                None => None,
            },
            SketchAction::Rotate(angle) => match sketch {
                Some(shape) => {
                    let shape_center = {
                        let mut gprops = ffi::GProp_GProps_ctor();
                        ffi::BRepGProp_VolumeProperties(&shape, gprops.pin_mut());

                        let centre_of_mass = ffi::GProp_GProps_CentreOfMass(&gprops);
                        Point2D::from_m(centre_of_mass.X(), centre_of_mass.Y())
                    };

                    let mut transform = ffi::new_transform();
                    transform.pin_mut().SetRotation(
                        &Axis {
                            origin: shape_center.to_3d(plane),
                            direction: plane.normal(),
                        }
                        .to_occt_ax1(),
                        angle.rad(),
                    );
                    let mut operation =
                        ffi::BRepBuilderAPI_Transform_ctor(&shape, &transform, false);
                    let new_shape = ffi::TopoDS_Shape_to_owned(operation.pin_mut().Shape());
                    Some(new_shape)
                }
                None => None,
            },
            SketchAction::Subtract(other) => match (sketch, other.to_occt(plane).ok()) {
                (None, None) => None,
                (None, Some(_)) => None,
                (Some(sketch), None) => Some(sketch),
                (Some(self_shape), Some(other_shape)) => {
                    let mut operation = ffi::BRepAlgoAPI_Cut_ctor(&self_shape, &other_shape);
                    Some(ffi::TopoDS_Shape_to_owned(operation.pin_mut().Shape()))
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Cuboid, Cylinder, Path, Point2D, Point3D, Rectangle, sketches::primitives::Circle,
    };

    use super::*;

    #[test]
    fn eq_both_rectangles() {
        assert_eq!(
            Rectangle::from_dim(Length::from_m(1.), Length::from_m(1.)),
            Rectangle::from_dim(Length::from_m(1.), Length::from_m(1.)),
        )
    }

    #[test]
    fn ne_both_rectangles() {
        assert_ne!(
            Rectangle::from_dim(Length::from_m(1.), Length::from_m(1.)),
            Rectangle::from_dim(Length::from_m(1.), Length::from_m(1.1)),
        )
    }

    #[test]
    fn eq_both_rectangles_not_at_origin() {
        assert_eq!(
            Rectangle::from_dim(Length::from_m(1.), Length::from_m(1.))
                .move_to(Point2D::from_m(2., 2.)),
            Rectangle::from_dim(Length::from_m(1.), Length::from_m(1.))
                .move_to(Point2D::from_m(2., 2.)),
        )
    }

    #[test]
    fn ne_both_rectangles_not_at_origin() {
        assert_ne!(
            Rectangle::from_dim(Length::from_m(1.), Length::from_m(1.))
                .move_to(Point2D::from_m(2., 2.)),
            Rectangle::from_dim(Length::from_m(1.), Length::from_m(1.))
                .move_to(Point2D::from_m(3., 3.)),
        )
    }

    #[test]
    fn ne_different_sketches() {
        assert_ne!(
            Rectangle::from_dim(Length::from_m(1), Length::from_m(1))
                .move_to(Point2D::from_m(2, 2)),
            Circle::from_radius(Length::from_m(1)).move_to(Point2D::from_m(2, 2)),
        )
    }

    #[test]
    fn intersect_non_overlapping() {
        let sketch1 = Rectangle::from_corners(Point2D::from_m(1., 1.), Point2D::from_m(2., 2.));
        let sketch2 = Rectangle::from_corners(Point2D::from_m(-1., -1.), Point2D::from_m(-2., -2.));
        assert!(sketch1.intersect(&sketch2).to_occt(&Plane::xy()).is_err())
    }

    #[test]
    fn extrude_empty_sketch() {
        let sketch = Sketch::empty();
        assert_eq!(
            sketch.extrude(&Plane::xy(), Length::from_m(5.)),
            Err(Error::EmptySketch)
        )
    }

    #[test]
    fn extrude_zero_thickness() {
        let sketch = Rectangle::from_dim(Length::from_m(1.), Length::from_m(2.));
        assert_eq!(
            sketch.extrude(&Plane::xy(), Length::zero()),
            Err(Error::EmptySketch)
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
            Ok(Cuboid::from_corners(
                Point3D::origin(),
                Point3D::from_m(1., 3., 2.)
            ))
        )
    }

    #[test]
    fn extrude_cylinder() {
        let sketch = Circle::from_radius(Length::from_m(1.));
        assert_eq!(
            sketch.extrude(&Plane::xy(), Length::from_m(2.)),
            Ok(
                Cylinder::from_radius(Length::from_m(1.), Length::from_m(2.))
                    .move_to(Point3D::from_m(0., 0., 1.))
            )
        )
    }
}
