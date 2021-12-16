use std::collections::HashMap;
use std::fs;

use crate::repos::{config::Config, errors::ConfigError};
use super::dependencies;

///
/// Kind of the package
///
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum PkgKind {
    Binary,
    Source,
}


/**
 * 
    Package (mandatory)
    Source
    Version (mandatory)
    Section (recommended)
    Priority (recommended)
    Architecture (mandatory)
    Essential
    Depends et al
    Installed-Size
    Maintainer (mandatory)
    Description (mandatory)
    Homepage
    Built-Using
*/
///
/// Debian's control file (mandatory fields)
///
// TODO: Add package priority
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlFile {
    pub package: String,
    // pub priority: PkgPriority
    pub version: String,
    pub architecture: String,
    pub maintainer: String,
    pub description: String,
    pub depends: Option<Vec<ControlFile>>,
    pub filename: String,
    pub size: String,
    pub md5sum: String,
    pub sha1: String,
    pub sha256: String,
    pub sha512: String
}

// TODO: Improve this in the future
impl ControlFile {
    pub fn new(config: &Config, file: &str) -> Result<Self, ConfigError> {
        let contents = fs::read_to_string(file)?;
        // println!("GOT : {}", contents);
        let mut map: HashMap<Option<String>, Option<String>> = HashMap::new();

        // FIXME: Find a better way of doing it
        for line in contents.lines() {
            let line = line.trim();
            let values = line.splitn(2, ":").map(|line| line.to_owned()).collect::<Vec<_>>();
            map.insert(
                if let Some(v) = values.get(0) {
                    Some(v.to_owned())
                } else {
                    None
                },

                if let Some(v) = values.get(1) {
                    Some(v.to_owned())
                } else {
                    None
                }
            );
        };

        let depends = Self::split_deps(Some(&Self::try_get(&map, "Depends")?)); /* Self::split_deps(map.get("Depends"));*/

        let result = Self {
            package: Self::try_get(&map, "Package")?,
            version: Self::try_get(&map, "Version")?,
            architecture: Self::try_get(&map, "Architecture")?,
            maintainer: Self::try_get(&map, "Maintainer")?,
            description: Self::try_get(&map, "Description")?,
            depends: None,
            // Should be like the others
            // But, when reading /var/lib/dpkg/status it does not have those fields
            filename: Self::try_get(&map, "Filename").unwrap_or_default(),
            size: Self::try_get(&map, "Size").unwrap_or_default(),
            md5sum: Self::try_get(&map, "MD5sum").unwrap_or_default(),
            sha1: Self::try_get(&map, "SHA1").unwrap_or_default(),
            sha256: Self::try_get(&map, "SHA256").unwrap_or_default(),
            sha512: Self::try_get(&map, "SHA512").unwrap_or_default(),
        };

        match depends {
            Some(v) => 
                Ok (
                    Self {
                        depends: dependencies::parse_dependencies(config, Some(v)),
                        ..result
                    },
                ),
            None => {
                Ok (
                    result
                )
            }
        }
    }

    pub fn from(config: &Config, contents: &str) -> Result<Self, ConfigError> {
        // println!("GOT : {}", contents);
        let mut map: HashMap<Option<String>, Option<String>> = HashMap::new();

        // FIXME: Find a better way of doing it
        for line in contents.lines() {
            let line = line.trim();
            let values = line.splitn(2, ":").map(|line| line.to_owned()).collect::<Vec<_>>();
            map.insert(
                if let Some(v) = values.get(0) {
                    Some(v.to_owned())
                } else {
                    None
                },

                if let Some(v) = values.get(1) {
                    Some(v.to_owned())
                } else {
                    None
                }
            );
        };

        let depends = Self::split_deps(Some(&Self::try_get(&map, "Depends")?)); /* Self::split_deps(map.get("Depends"));*/

        let result = Self {
            package: Self::try_get(&map, "Package")?,
            version: Self::try_get(&map, "Version")?,
            architecture: Self::try_get(&map, "Architecture")?,
            maintainer: Self::try_get(&map, "Maintainer")?,
            description: Self::try_get(&map, "Description")?,
            depends: None,
            // Should be like the others
            // But, when reading /var/lib/dpkg/status it does not have those fields
            filename: Self::try_get(&map, "Filename").unwrap_or_default(),
            size: Self::try_get(&map, "Size").unwrap_or_default(),
            md5sum: Self::try_get(&map, "MD5sum").unwrap_or_default(),
            sha1: Self::try_get(&map, "SHA1").unwrap_or_default(),
            sha256: Self::try_get(&map, "SHA256").unwrap_or_default(),
            sha512: Self::try_get(&map, "SHA512").unwrap_or_default(),
        };

        match depends {
            Some(v) => 
                Ok (
                    Self {
                        depends: dependencies::parse_dependencies(config, Some(v)),
                        ..result
                    },
                ),
            None => {
                Ok (
                    result
                )
            }
        }
    }

    // TODO: Maybe I need to make this easier to read
    fn try_get(hashmap: &HashMap<Option<String>, Option<String>>, field: &str) -> Result<String, ConfigError> {
        let value = hashmap.get(&Some(field.to_owned()));
        if value.is_none() {
            Err(ConfigError::Error("Invalid debain's control file! Missing \"Package\" field".to_owned()))
        } else {
            Ok (value.unwrap().as_ref().unwrap().clone().trim().to_owned())
        }
    }

    fn split_deps(dependencies: Option<&String>) -> Option<Vec<String>> {
        if let Some(val) = dependencies {
            let val = val
                .split(",")
                .map(|d| d.trim().to_owned())
                .collect::<Vec<_>>();
            Some(val)
        } else {
            None
        }
    }

    pub fn set_filename(&mut self, filename: &str) {
        self.filename = filename.to_owned();
    }
}

/// 
/// Debian binary package format structure
///
#[derive(Debug, Clone)]
pub struct DebPackage {
    pub control: ControlFile,
    pub kind: PkgKind,
}

impl DebPackage {
    pub fn new(config: &Config, file: &str, kind: PkgKind) -> Result<Self, ConfigError> {
        Ok(
            DebPackage {
                control: ControlFile::new(config, file)?,

                kind,
            }
        )
    }
}