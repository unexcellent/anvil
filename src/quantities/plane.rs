use crate::Error;

use super::{Axis, Point3D, dir3::Dir3};

/// A 2D plane in 3D space.
#[derive(Debug, PartialEq, Clone)]
pub struct Plane(Point3D, Dir3, Dir3);
impl Plane {
    /// Construct the `Plane` spaned by the x and y axes.
    pub fn xy() -> Self {
        Self::new(Point3D::origin(), (1., 0., 0.), (0., 1., 0.)).expect("error in axis def")
    }
    /// Construct the `Plane` spaned by the x and z axes.
    pub fn xz() -> Self {
        Self::new(Point3D::origin(), (1., 0., 0.), (0., 0., 1.)).expect("error in axis def")
    }
    /// Construct the `Plane` spaned by the y and z axes.
    pub fn yz() -> Self {
        Self::new(Point3D::origin(), (0., 1., 0.), (0., 0., 1.)).expect("error in axis def")
    }

    /// Construct a `Plane` from a point and two orthogonal vectors.
    ///
    /// `x_axis` defines the direction of the x-axis inside the plane. `y_axis` defines the
    /// direction of the y-axis inside the plane. Both are used to project from the local 2D
    /// coordinate system to the global coordinate system. If the two axes are not orthogonal,
    /// an `Err(Error::VectorsNotOrthogonal)` is returned.
    pub fn new(
        origin: Point3D,
        x_axis: (f64, f64, f64),
        y_axis: (f64, f64, f64),
    ) -> Result<Self, Error> {
        let x_axis = Dir3::from(x_axis);
        let y_axis = Dir3::from(y_axis);

        let axes_are_orthogonal = x_axis.dot(y_axis) < 1e-9;
        if !axes_are_orthogonal {
            return Err(Error::VectorsNotOrthogonal(x_axis, y_axis));
        }
        Ok(Self(origin, x_axis.normalize()?, y_axis.normalize()?))
    }

    /// Return the origin point of this `Plane`.
    pub fn origin(&self) -> Point3D {
        self.0
    }
    /// Return a the x-axis direction of this `Plane`.
    pub fn x_axis(&self) -> Dir3 {
        self.1
    }
    /// Return a the y-axis direction of this `Plane`.
    pub fn y_axis(&self) -> Dir3 {
        self.2
    }
    /// Return a `Dir3` that is orthogonal to this plane.
    pub fn normal(&self) -> Dir3 {
        self.x_axis().cross(self.y_axis())
    }
    /// Return an `Axis` that is orthogonal to this plane and crosses its origin.
    pub fn normal_axis(&self) -> Axis {
        Axis {
            origin: self.origin(),
            direction: self.normal(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_unnormalized() {
        let plane = Plane::new(Point3D::origin(), (2., 0., 0.), (0., 2., 0.));
        assert_eq!(
            plane,
            Ok(Plane(
                Point3D::origin(),
                Dir3::from((1., 0., 0.)),
                Dir3::from((0., 1., 0.))
            ))
        )
    }

    #[test]
    fn new_non_orthogonal_vector() {
        let plane = Plane::new(Point3D::origin(), (2., 0., 0.), (0., 2., 0.));
        assert_eq!(
            plane,
            Ok(Plane(
                Point3D::origin(),
                Dir3::from((1., 0., 0.)),
                Dir3::from((0., 1., 0.))
            ))
        )
    }
}
