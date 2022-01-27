use anyhow::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum InstallError {
    InvalidPackage   { pkg: String, err: Option<Error> },
    BrokenPackage    { pkg: String, err: Option<Error> },
    WrongVersion     { pkg: String, reqv: String, curv: String },
    UnexError        { msg: String, err: Option<Error> },
    AlreadyInstalled ( String ),
    Breaks           ( String ),
    NetworkingError  { err: Error },
    UnexInterrupt    { err: Error },
    UserInterrupt,
}

#[derive(Debug)]
pub enum RemoveError {
    NotFoundError    ( String ),
    UnexError        { msg: String, err: Option<Error> },
}

#[derive(Debug)]
pub enum ScriptsError {
    PreInstError,
    PostInstError,
    PreRmError,
    PostRmError,
    UnexError { msg: String, err: Option<Error> },
}

#[derive(Debug)]
pub enum SignatureError {
    MD5    { rs: String, ex: String },
    SHA1   { rs: String, ex: String },
    SHA256 { rs: String, ex: String },
    SHA512 { rs: String, ex: String },
}

#[derive(Debug)]
pub enum ConfigError {
    ChangeConfig  ( String ),
    SetupError    ( Error ),
    UnexError     { msg: String, err: Option<Error> },
}

#[derive(Debug)]
pub enum CacheError {
    NotFoundError { pkg: String, cache: String },
    UnexError     { msg: String, err: Option<Error> },
    NoCache       ( String ),
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
            InstallError::Breaks ( pkg ) => write!(f, "Package {:?} can break others", pkg),
            InstallError::WrongVersion { pkg, reqv, curv } => write!(f, "Package \"{}({})\" does not satisfy \"{}({})\"", pkg, curv, pkg, reqv),
            InstallError::NetworkingError { err } => write!(f, "Networking Error :: {:?}", err),
            InstallError::UserInterrupt => write!(f, "Installation was interrupted by the user"),
            InstallError::UnexInterrupt { err } => write!(f, "Installation was unexpected interrupted :: error {:?}", err),
        }
    }
}

impl Display for RemoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RemoveError::NotFoundError ( pkg ) => write!(f, "Could not remove {:?} due files were not found", pkg),
            RemoveError::UnexError { msg, err } => write!(f, "Unexpected Error {:?} :: {:?}", msg, err),
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
        match self {
            ConfigError::SetupError ( err ) => write!(f, "Failed to complete setup :: error {:?}", err),
            ConfigError::UnexError { msg, err } => write!(f, "Unexpected Error {:?} :: {:?}", msg, err),
            ConfigError::ChangeConfig ( config ) => write!(f, "Go to {:?} and change the values manually", config),
        }
    }
}

impl Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheError::NotFoundError { pkg, cache } => write!(f, "{:?} was not found at {:?}", pkg, cache),
            CacheError::UnexError { msg, err } => write!(f, "Unexpected Error {:?} :: {:?}", msg, err),
            CacheError::NoCache ( cache ) => write!(f, "No cache file was found at {:?}", cache),
        }
    }
}

impl Display for ScriptsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScriptsError::PreInstError => write!(f, "Could not execute PREINST the script"),
            ScriptsError::PostInstError => write!(f, "Could not execute POSTINST the script"),
            ScriptsError::PreRmError => write!(f, "Could not execute PRERM the script"),
            ScriptsError::PostRmError => write!(f, "Could not execute POSTRM the script"),
            ScriptsError::UnexError { msg, err } => write!(f, "Unexpected Error {:?} :: {:?}", msg, err),
        }
    }
}