use cxx::UniquePtr;
use opencascade_sys::ffi;

use crate::Error;

use super::{Dir3D, Point3D};

/// An axis in 3D space.
#[derive(Debug, PartialEq, Clone)]
pub struct Axis {
    /// A point contained in the axis.
    pub origin: Point3D,
    /// The directional vector of the axis.
    pub direction: Dir3D,
}
impl Axis {
    /// Construct an `Axis`.
    pub fn new(origin: Point3D, direction: Dir3D) -> Self {
        Self { origin, direction }
    }

    /// Construct an `Axis` that lies between two points.
    ///
    /// This constructor can return an error if the two points are at the same location.
    ///
    /// ```rust
    /// use anvil::{Axis, point, Dir3D};
    ///
    /// assert_eq!(
    ///     Axis::between(point!(1 m, 1 m, 1 m), point!(2 m, 1 m, 1 m)),
    ///     Ok(Axis {
    ///         origin: point!(1 m, 1 m, 1 m),
    ///         direction: Dir3D::try_from(1., 0., 0.).expect("")
    ///     })
    /// );
    /// assert!(Axis::between(point!(1 m, 1 m, 1 m), point!(1 m, 1 m, 1 m)).is_err())
    /// ```
    pub fn between(origin: Point3D, other: Point3D) -> Result<Self, Error> {
        let direction = other.direction_from(&origin)?;
        Ok(Self { origin, direction })
    }

    /// Return the axis identical to the x-axis at the origin.
    pub fn x() -> Self {
        Axis::new(Point3D::origin(), Dir3D::try_from(1., 0., 0.).expect(""))
    }
    /// Return the axis identical to the y-axis at the origin.
    pub fn y() -> Self {
        Axis::new(Point3D::origin(), Dir3D::try_from(0., 1., 0.).expect(""))
    }
    /// Return the axis identical to the z-axis at the origin.
    pub fn z() -> Self {
        Axis::new(Point3D::origin(), Dir3D::try_from(0., 0., 1.).expect(""))
    }
    /// Return the axis identical to the x-axis at the origin in reverse direction.
    pub fn neg_x() -> Self {
        Axis::new(Point3D::origin(), Dir3D::try_from(-1., 0., 0.).expect(""))
    }
    /// Return the axis identical to the y-axis at the origin in reverse direction.
    pub fn neg_y() -> Self {
        Axis::new(Point3D::origin(), Dir3D::try_from(0., -1., 0.).expect(""))
    }
    /// Return the axis identical to the z-axis at the origin in reverse direction.
    pub fn neg_z() -> Self {
        Axis::new(Point3D::origin(), Dir3D::try_from(0., 0., -1.).expect(""))
    }

    pub(crate) fn to_occt_ax1(&self) -> UniquePtr<ffi::gp_Ax1> {
        ffi::gp_Ax1_ctor(&self.origin.to_occt_point(), &self.direction.to_occt_dir())
    }
    pub(crate) fn to_occt_ax2(&self) -> UniquePtr<ffi::gp_Ax2> {
        ffi::gp_Ax2_ctor(&self.origin.to_occt_point(), &self.direction.to_occt_dir())
    }
}
