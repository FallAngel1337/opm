// use std::error::Error;
use std::fmt::{self, Display};
use std::io::Error as ioError;
use rusqlite::Error as sqliteError;
use reqwest::Error as reqwestError;

// TODO: Make better erros and create more
#[derive(Debug)]
pub enum InstallError {
    InvalidPackage(String),
    IoError(ioError, String),
    NetworkingError(reqwestError, String),
    DataBaseError(sqliteError, String)
}

impl Display for InstallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstallError::InvalidPackage(msg) => write!(f, "Invalid Package => {}", msg),
            InstallError::IoError(err, msg) => write!(f, "I/O Error => {} :: {}", msg, err),
            InstallError::DataBaseError(err, msg) => write!(f, "DataBase Error => {} :: {}", msg, err),
            InstallError::NetworkingError(err, msg) => write!(f, "Networking Error => {} :: {}", msg, err)
        }
    }
}

impl From<ioError> for InstallError {
    fn from(err: ioError) -> Self {
        InstallError::IoError(err, "NO_MSG".to_string())
    }
}

impl From<sqliteError> for InstallError {
    fn from(err: sqliteError) -> Self {
        InstallError::DataBaseError(err, "NO_MSG".to_string())
    }
}

impl From<reqwestError> for InstallError {
    fn from(err: reqwestError) -> Self {
        InstallError::NetworkingError(err, "NO_MSG".to_string())
    }
}