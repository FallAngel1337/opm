use anyhow::Error;
use std::fmt::{self, Display};

// TODO: Make better erros and create more
#[derive(Debug)]
pub enum InstallError {
    InvalidPackage { err: Error },
    NetworkingError { err: Error },
    AlreadyInstalled,
    NotFoundError(String),
    Error(String),
}

#[derive(Debug)]
pub struct SetupError {
    pub msg: String
}

#[derive(Debug)]
pub struct  ConfigError {
    pub msg: String
}

#[derive(Debug)]
pub struct CacheError {
    pub msg: String
}

impl std::error::Error for InstallError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

impl Display for InstallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstallError::InvalidPackage { err } => write!(f, "Invalid Package :: \"{}\"", err),
            InstallError::NetworkingError { err } => write!(f, "Networking Error :: \"{}\"", err),
            InstallError::AlreadyInstalled => write!(f, "Package is already installed"),
            InstallError::NotFoundError(pkg) => write!(f, "Package {} was not found", pkg),
            InstallError::Error(msg) => write!(f, "Error during installation :: {}", msg)
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ConfigError :: {}", self.msg)
    }
}

impl std::error::Error for SetupError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

impl Display for SetupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SetupError :: {}", self.msg)
    }
}

impl std::error::Error for CacheError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

impl Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SetupError :: {}", self.msg)
    }
}