use std::{error::Error as StdError, fmt, path::PathBuf};

use crate::quantities::Vec3;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// Occurs when a function that requires a non-empty `Part` is called on an empty one.
    EmptyPart,

    /// Occurs when a function that requires a non-empty `Sketch` is called on an empty one.
    EmptySketch,

    /// Occurs when a `Part` could not be written to a .step file at a given path.
    StepWrite(PathBuf),

    /// Occurs when a `Part` could not be written to a .stl file at a given path.
    StlWrite(PathBuf),

    ZeroVector(Vec3),
    VectorsNotOrthogonal(Vec3, Vec3),
}
impl StdError for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Under construction")
    }
}
