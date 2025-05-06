#![allow(clippy::approx_constant)]
mod errors;
mod parts;
mod quantities;

pub use errors::Error;
pub use parts::{
    Part,
    primitives::{Cuboid, Cylinder, Sphere},
};
pub use quantities::{Length, Point3D};
