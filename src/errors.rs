use std::{error::Error as StdError, fmt, path::PathBuf};

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    StepWrite(PathBuf),
}
impl StdError for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Under construction")
    }
}
