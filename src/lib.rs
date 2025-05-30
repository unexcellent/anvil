#![doc = "A CAD engine."]
#![allow(clippy::approx_constant)]
#![warn(missing_docs)]

mod errors;
mod parts;
mod quantities;
mod sketches;

pub use errors::Error;
pub use parts::{
    primitives::{Cuboid, Cylinder, Sphere},
    Part,
};
pub use quantities::{Angle, Axis, Length, Plane, Point2D, Point3D, Vec3};
pub use sketches::{
    primitives::{Circle, Rectangle},
    Edge, Path, Sketch,
};
