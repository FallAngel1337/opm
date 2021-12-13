use std::collections::HashMap;
use std::fs;
use std::io::Error;

use crate::repos::config::Config;
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlFile {
    pub package: String,
    pub version: String,
    pub architecture: String,
    pub maintainer: String,
    pub description: String,
    pub depends: Option<Vec<ControlFile>>,
    pub filename: String,
}

// TODO: Improve this in the future
impl ControlFile {
    pub fn new(config: &Config, file: &str) -> Result<Self, Error> {
        let contents = fs::read_to_string(file)?;

        let mut map: HashMap<String, String> = HashMap::new();

        for line in contents.lines() {
            let values = line.split(":").collect::<Vec<&str>>();
            map.insert(String::from(*values.get(0).unwrap_or(&"NONE")), String::from(*values.get(1).unwrap_or(&"NONE")));
        };

        let depends = Self::split_deps(map.get("Depends"));

        Ok(
            Self {
                package: map.get("Package").unwrap_or(&String::from("NONE")).trim().to_owned(),
                version: map.get("Version").unwrap_or(&String::from("NONE")).trim().to_owned(),
                architecture: map.get("Architecture").unwrap_or(&String::from("NONE")).trim().to_owned(),
                maintainer: map.get("Maintainer").unwrap_or(&String::from("NONE")).trim().to_owned(),
                description: map.get("Description").unwrap_or(&String::from("NONE")).trim().to_owned(),
                depends: dependencies::parse_dependencies(config, depends),
                filename: map.get("Filename").unwrap_or(&String::from("NONE")).trim().to_owned(),
            }
        )
    }

    pub fn from(config: &Config, contents: &str) -> Self {
        let mut map: HashMap<String, String> = HashMap::new();

        for line in contents.lines() {
            let values = line.splitn(2, ":").collect::<Vec<&str>>();
            map.insert(String::from(*values.get(0).unwrap_or(&"NONE")), String::from(*values.get(1).unwrap_or(&"NONE")));
        };

        let depends = Self::split_deps(map.get("Depends"));
        let result = Self {
            package: map.get("Package").unwrap_or(&String::from("NONE")).trim().to_owned(),
            version: map.get("Version").unwrap_or(&String::from("NONE")).trim().to_owned(),
            architecture: map.get("Architecture").unwrap_or(&String::from("NONE")).trim().to_owned(),
            maintainer: map.get("Maintainer").unwrap_or(&String::from("NONE")).trim().to_owned(),
            description: map.get("Description").unwrap_or(&String::from("NONE")).trim().to_owned(),
            filename: map.get("Filename").unwrap_or(&String::from("NONE")).trim().to_owned(),
            depends: None,
        };

        match depends {
            Some(v) => 
                Self {
                    depends: dependencies::parse_dependencies(config, Some(v)),
                    ..result
                },
            None => {
                result
            }
        }
    }

    // Same as `from` but doesn't parse the dependencies
    pub fn parse_no_deps(contents: &str) -> Option<Self>{
        let mut map: HashMap<String, String> = HashMap::new();

        for line in contents.lines() {
            let values = line.splitn(2, ":").collect::<Vec<&str>>();
            map.insert(String::from(*values.get(0).unwrap_or(&"NONE")), String::from(*values.get(1).unwrap_or(&"NONE")));
        };
        
        Some(
            Self {
                package: map.get("Package").unwrap_or(&String::from("NONE")).trim().to_owned(),
                version: map.get("Version").unwrap_or(&String::from("NONE")).trim().to_owned(),
                architecture: map.get("Architecture").unwrap_or(&String::from("NONE")).trim().to_owned(),
                maintainer: map.get("Maintainer").unwrap_or(&String::from("NONE")).trim().to_owned(),
                description: map.get("Description").unwrap_or(&String::from("NONE")).trim().to_owned(),
                depends: None,
                filename: map.get("Filename").unwrap_or(&String::from("NONE")).trim().to_owned(),
            }
        )
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
    pub signature: String,
    pub kind: PkgKind,
}

impl DebPackage {
    pub fn new(config: &Config, file: &str, kind: PkgKind, signature: String) -> Result<Self, Error> {
        Ok(
            DebPackage {
                control: ControlFile::new(config, file)?,
                signature,
                kind
            }
        )
    }
}
