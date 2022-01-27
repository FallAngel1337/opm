use anyhow::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum InstallError {
    InvalidPackage   { pkg: String, err: Option<Error> },
    BrokenPackage    { pkg: String, err: Option<Error> },
    UnexError        { msg: String, err: Option<Error> },
    AlreadyInstalled ( String ),
    NotFoundError    ( String ),
    Breaks           ( String ),
    NetworkingError  { err: Error },
    UnexInterrupt    { err: Error },
    UserInterrupt,
}

#[derive(Debug)]
pub enum SignatureError {
    MD5    { rs: String, ex: String },
    SHA1   { rs: String, ex: String },
    SHA256 { rs: String, ex: String },
    SHA512 { rs: String, ex: String },
}

#[derive(Debug)]
pub struct ConfigError {
    pub msg: String,
    pub err: Option<Error>,
}

#[derive(Debug)]
pub struct CacheError {
    pub msg: String,
    pub err: Option<Error>,
}

impl std::error::Error for InstallError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

impl Display for InstallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstallError::InvalidPackage { pkg, err } => write!(f, "{:?} is a invalid package :: error {:?}", pkg, err),
            InstallError::BrokenPackage { pkg, err } => write!(f, "Perhaps {:?} is broken due some missing files :: error {:?}", pkg, err),
            InstallError::AlreadyInstalled ( pkg ) => write!(f, "{:?} is already installed", pkg),
            InstallError::UnexError { msg, err }  => write!(f, "Unexpected Error {:?} :: {:?}", msg, err),
            InstallError::NotFoundError ( pkg ) => write!(f, "Package {:?} was not found in cache", pkg),
            InstallError::Breaks ( pkg ) => write!(f, "Package {:?} can break others", pkg),
            InstallError::NetworkingError { err } => write!(f, "Networking Error :: {:?}", err),
            InstallError::UserInterrupt => write!(f, "Installation was interrupted by the user"),
            InstallError::UnexInterrupt { err } => write!(f, "Installation was unexpected interrupted :: error {:?}", err),
        }
    }
}

impl Display for SignatureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignatureError::MD5 { rs, ex } => write!(f, "Mismatched MD5 Hash got {:?} expected {:?}", rs, ex),
            SignatureError::SHA1 { rs, ex } => write!(f, "Mismatched SHA1 Hash got {:?} expected {:?}", rs, ex),
            SignatureError::SHA256 { rs, ex } => write!(f, "Mismatched SHA256 Hash got {:?} expected {:?}", rs, ex),
            SignatureError::SHA512 { rs, ex } => write!(f, "Mismatched SHA512 Hash got {:?} expected {}", rs, ex),
        }
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ConfigError {:?} :: {:?}", self.msg, self.err)
    }
}

impl Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CacheError {:?} :: {:?}", self.msg, self.err)
    }
}