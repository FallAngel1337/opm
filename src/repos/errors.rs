// use std::error::Error;
use std::fmt::{self, Display};
use std::io::Error as ioError;

#[derive(Debug)]
pub enum InstallError {
    InvalidPackage,
    IoError(ioError)
}

impl Display for InstallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstallError::InvalidPackage => write!(f, "Invalid Package"),
            InstallError::IoError(err) => write!(f, "I/O Error :: {}", err)
        }
    }
}

impl From<ioError> for InstallError {
    fn from(err: ioError) -> Self {
        InstallError::IoError(err)
    }
}