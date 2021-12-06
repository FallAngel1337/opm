use std::collections::HashMap;
use std::fs;
use std::io::Error;

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
#[derive(Debug, Clone)]
pub struct ControlFile { // We could improve by using lifetimes
    pub package: String,
    pub version: String,
    pub architecture: String,
    pub maintainer: String,
    pub description: String,
    pub depends: String
}

// TODO: Improve this in the future
impl ControlFile {
    fn new(file: &str) -> Result<Self, Error> {
        let contents = fs::read_to_string(file)?;

        let mut map: HashMap<String, String> = HashMap::new();

        for line in contents.lines() {
            let values = line.split(":").collect::<Vec<&str>>();
            map.insert(String::from(*values.get(0).unwrap_or(&"NONE")), String::from(*values.get(1).unwrap_or(&"NONE")));
        };

        Ok(
            ControlFile {
                package: map.get("Package").unwrap_or(&String::from("NONE")).clone(),
                version: map.get("Version").unwrap_or(&String::from("NONE")).clone(),
                architecture: map.get("Architecture").unwrap_or(&String::from("NONE")).clone(),
                maintainer: map.get("Maintainer").unwrap_or(&String::from("NONE")).clone(),
                description: map.get("Description").unwrap_or(&String::from("NONE")).clone(),
                depends: map.get("Depends").unwrap_or(&String::from("NONE")).clone(),
            }
        )
    }

    pub fn from(contents: &str) -> Result<Self, Error> {
        let mut map: HashMap<String, String> = HashMap::new();

        for line in contents.lines() {
            let values = line.splitn(2, ":").collect::<Vec<&str>>();
            map.insert(String::from(*values.get(0).unwrap_or(&"NONE")), String::from(*values.get(1).unwrap_or(&"NONE")));
        };

        Ok(
            ControlFile {
                package: map.get("Package").unwrap_or(&String::from("NONE")).clone(),
                version: map.get("Version").unwrap_or(&String::from("NONE")).clone(),
                architecture: map.get("Architecture").unwrap_or(&String::from("NONE")).clone(),
                maintainer: map.get("Maintainer").unwrap_or(&String::from("NONE")).clone(),
                description: map.get("Description").unwrap_or(&String::from("NONE")).clone(),
                depends: map.get("Depends").unwrap_or(&String::from("NONE")).clone(),
            }
        )
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
    pub fn new(file: &str, kind: PkgKind, signature: String) -> Result<Self, Error> {
        Ok(
            DebPackage {
                control: ControlFile::new(file)?,
                signature,
                kind
            }
        )
    }
}
