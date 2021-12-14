// use std::error::Error;
use std::fmt::{self, Display};
use std::io::Error as ioError;
use rusqlite::Error as sqliteError;
use reqwest::Error as reqwestError;

// TODO: Make better erros and create more
#[derive(Debug)]
pub enum InstallError {
    InvalidPackage(String),
    IoError(String),
    NetworkingError(String),
    DataBaseError(String)
}

pub enum SetupError {
    Error(String)
}

impl Display for InstallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstallError::InvalidPackage(msg) => write!(f, "Invalid Package => {}", msg),
            InstallError::IoError(msg) => write!(f, "I/O Error => {}", msg),
            InstallError::DataBaseError(msg) => write!(f, "DataBase Error => {}", msg),
            InstallError::NetworkingError(msg) => write!(f, "Networking Error => {}", msg)
        }
    }
}

impl From<ioError> for InstallError {
    fn from(err: ioError) -> Self {
        InstallError::IoError(err.to_string())
    }
}

impl From<sqliteError> for InstallError {
    fn from(err: sqliteError) -> Self {
        InstallError::DataBaseError(err.to_string())
    }
}

impl From<reqwestError> for InstallError {
    fn from(err: reqwestError) -> Self {
        InstallError::NetworkingError(err.to_string())
    }
}

impl Display for SetupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SetupError::Error(msg) => write!(f, "{}", msg)
        }
    }
}

impl<E: std::error::Error + 'static> From<E> for SetupError {
    fn from(error: E) -> Self {
        SetupError::Error(error.to_string())
    }
}