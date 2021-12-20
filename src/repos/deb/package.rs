use std::collections::HashMap;
use std::fs;

use crate::repos::errors::ConfigError;

///
/// Kind of the package
///
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum PkgKind {
    Binary,
    Source,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PkgPriority {
    Required,
    Important,
    Standard,
    Optional,
    Extra, // Deprecated, but here for compatibility issues
}

///
/// Debian's control file (mandatory fields)
///
// TODO: Add package priority
#[derive(Debug, Clone, PartialEq)]
pub struct ControlFile {
    pub package: String,
    pub version: String,
    pub priority: PkgPriority,
    pub architecture: String,
    pub maintainer: String,
    pub description: String,
    pub depends: Option<Vec<String>>,
    pub filename: String,
    pub size: String,
    pub md5sum: String,
    pub sha1: String,
    pub sha256: String,
    pub sha512: String
}

impl PkgPriority {
    fn get_priority(p: &str) -> Self {
        match p {
            "required" => PkgPriority::Required,
            "important" => PkgPriority::Important,
            "standard" => PkgPriority::Standard,
            "optional" => PkgPriority::Optional,
            "extra" => PkgPriority::Extra,
            _ => panic!("Invalid priority")
        }
    }
}

// TODO: Improve this in the future
impl ControlFile {
    pub fn new(file: &str) -> Result<Self, ConfigError> {
        let contents = fs::read_to_string(file)?;
        Self::from(&contents)
    }

    pub fn from(contents: &str) -> Result<Self, ConfigError> {
        let mut map: HashMap<Option<String>, Option<String>> = HashMap::new();

        for line in contents.lines() {
            let line = line.trim();
            let values = line.splitn(2, ':').map(|line| line.to_owned()).collect::<Vec<_>>();
            map.insert(
                values.get(0).map(|v| v.to_owned()),
                values.get(1).map(|v| v.to_owned())
            );
        }

        let result = Self {
            package: Self::try_get(&map, "Package")?,
            version: Self::try_get(&map, "Version")?,
            priority: PkgPriority::get_priority(&Self::try_get(&map, "Priority")?),
            architecture: Self::try_get(&map, "Architecture")?,
            maintainer: Self::try_get(&map, "Maintainer")?,
            description: Self::try_get(&map, "Description")?,
            depends: Self::split_deps(Some(&Self::try_get(&map, "Depends")?)),
            // Should be like the others
            // But, when reading /var/lib/dpkg/status it does not have those fields
            filename: Self::try_get(&map, "Filename").unwrap_or_default(),
            size: Self::try_get(&map, "Size").unwrap_or_default(),
            md5sum: Self::try_get(&map, "MD5sum").unwrap_or_default(),
            sha1: Self::try_get(&map, "SHA1").unwrap_or_default(),
            sha256: Self::try_get(&map, "SHA256").unwrap_or_default(),
            sha512: Self::try_get(&map, "SHA512").unwrap_or_default(),
        };        

        Ok(result)
    }

    // TODO: Maybe I need to make this easier to read
    fn try_get(hashmap: &HashMap<Option<String>, Option<String>>, field: &str) -> Result<String, ConfigError> {
        let value = hashmap.get(&Some(field.to_owned()));
        if let Some(v) = value {
            Ok (v.as_ref().unwrap().trim().to_owned())
        } else {
            Err(ConfigError::Error(
                    format!("Invalid debain's control file! Missing \"{}\" field", field)
                )
            )
        }
    }

    fn split_deps(dependencies: Option<&String>) -> Option<Vec<String>> {
        if let Some(val) = dependencies {
            let val = val
                .split(',')
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
#[derive(Debug, Clone, PartialEq)]
pub struct DebPackage {
    pub control: ControlFile,
    pub kind: PkgKind,
}

impl DebPackage {
    pub fn new(file: &str, kind: PkgKind) -> Result<Self, ConfigError> {
        Ok(
            DebPackage {
                control: ControlFile::new(file)?,

                kind,
            }
        )
    }
}