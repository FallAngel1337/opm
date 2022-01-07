use anyhow::Error;
use std::fmt::{self, Display};

// TODO: Make better erros and create more
#[derive(Debug)]
pub enum InstallError {
    InvalidPackage { err: Error },
    NetworkingError { err: Error },
    AlreadyInstalled,
    Interrupted,
    BrokenPackage(String),
    Breaks(String),
    NotFoundError(String),
    Error(String)
}

#[derive(Debug)]
pub enum SignatureError {
    MD5(String, String),
    SHA1(String, String),
    SHA256(String, String),
    SHA512(String, String)
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
            InstallError::Interrupted => write!(f, "Installation got interrupted by the user"),
            InstallError::BrokenPackage(msg) => write!(f, "Perhaps {} is broken", msg),
            InstallError::Breaks(pkg) => write!(f, "Package {} can break others", pkg),
            InstallError::NotFoundError(pkg) => write!(f, "Package {} was not found", pkg),
            InstallError::Error(msg) => write!(f, "Error during installation :: {}", msg)
        }
    }
}

impl Display for SignatureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignatureError::MD5(rs, ex) => write!(f, "Mismatched MD5 Hash got {} expected {}", rs, ex),
            SignatureError::SHA1(rs, ex) => write!(f, "Mismatched SHA1 Hash got {} expected {}", rs, ex),
            SignatureError::SHA256(rs, ex)=> write!(f, "Mismatched SHA256 Hash got {} expected {}", rs, ex),
            SignatureError::SHA512(rs, ex) => write!(f, "Mismatched SHA512 Hash got {} expected {}", rs, ex),
        }
    }
}

impl std::error::Error for SignatureError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
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
        write!(f, "CacheError :: {}", self.msg)
    }
}