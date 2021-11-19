// use std::error::Error;
use std::fmt::{self, Display};
use std::io::Error as ioError;
use rusqlite::Error as sqliteError;

#[derive(Debug)]
pub enum InstallError {
    InvalidPackage,
    IoError(ioError),
    DataBaseError(sqliteError)
}

impl Display for InstallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstallError::InvalidPackage => write!(f, "Invalid Package"),
            InstallError::IoError(err) => write!(f, "I/O Error :: {}", err),
            InstallError::DataBaseError(err) => write!(f, "DataBase Error :: {}", err)
        }
    }
}

impl From<ioError> for InstallError {
    fn from(err: ioError) -> Self {
        InstallError::IoError(err)
    }
}

impl From<sqliteError> for InstallError {
    fn from(err: sqliteError) -> Self {
        InstallError::DataBaseError(err)
    }
}