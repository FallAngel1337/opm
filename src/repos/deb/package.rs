#![allow(unused)]
use anyhow::{self, Result, bail};
use crate::repos::{errors::ConfigError, config::Config};
use std::collections::HashMap;
use std::fs;

use super::{cache, dependencies::get_dependencies};


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
#[derive(Debug, Clone, PartialEq)]
pub struct ControlFile {
    pub package: String,
    pub version: String,
    pub priority: String,
    pub architecture: String,
    pub maintainer: String,
    pub description: String,
    pub depends: Option<Vec<ControlFile>>,
    pub recommends: Option<Vec<String>>,
    pub suggests: Option<Vec<String>>,
    pub enhances: Option<Vec<String>>,
    pub pre_depends: Option<Vec<String>>,
    pub breaks: Option<Vec<String>>,
    pub conflicts: Option<Vec<String>>,
    pub conffiles: Option<Vec<String>>,
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
            _ => PkgPriority::Optional
        }
    }
}

// TODO: Improve this in the future
impl ControlFile {
    pub fn new(config: &Config, contents: &str) -> Result<Self> {
        let mut map: HashMap<Option<String>, Option<String>> = HashMap::new();

        for line in contents.lines() {
            let line = line.trim();
            let values = line.splitn(2, ':').map(|line| line.to_owned()).collect::<Vec<_>>();
            map.insert(
                values.get(0).map(|v| v.to_owned()),
                values.get(1).map(|v| v.to_owned())
            );
        }

        Ok(
            Self {
                package: Self::try_get(&map, "Package")?,
                version: Self::try_get(&map, "Version")?,
                architecture: Self::try_get(&map, "Architecture")?,
                maintainer: Self::try_get(&map, "Maintainer")?,
                description: Self::try_get(&map, "Description")?,
                priority: Self::try_get(&map, "Priority").unwrap_or_default(),
                depends: None,
                recommends: None,
                suggests: None,
                enhances: None,
                pre_depends: None,
                breaks: None,
                conflicts: None,
                conffiles: None,
                filename: Self::try_get(&map, "Filename").unwrap_or_default(),
                size: Self::try_get(&map, "Size").unwrap_or_default(),
                md5sum: Self::try_get(&map, "MD5sum").unwrap_or_default(),
                sha1: Self::try_get(&map, "SHA1").unwrap_or_default(),
                sha256: Self::try_get(&map, "SHA256").unwrap_or_default(),
                sha512: Self::try_get(&map, "SHA512").unwrap_or_default(),
            }
        )
    }

    pub fn from_file(config: &Config, file: &str) -> Result<Self> {
        let contents = fs::read_to_string(file)?;
        Self::from(config, &contents)
    }

    pub fn from(config: &Config, contents: &str) -> Result<Self> {        
        let mut map: HashMap<Option<String>, Option<String>> = HashMap::new();

        for line in contents.lines() {
            let line = line.trim();
            let values = line.splitn(2, ':').map(|line| line.to_owned()).collect::<Vec<_>>();
            map.insert(
                values.get(0).map(|v| v.to_owned()),
                values.get(1).map(|v| v.to_owned())
            );
        }

        Ok(
            Self {
                package: Self::try_get(&map, "Package")?,
                version: Self::try_get(&map, "Version")?,
                architecture: Self::try_get(&map, "Architecture")?,
                maintainer: Self::try_get(&map, "Maintainer")?,
                description: Self::try_get(&map, "Description")?,
                // Should be like the others
                // But, when reading /var/lib/dpkg/status it does not have those fields
                priority: Self::try_get(&map, "Priority").unwrap_or_default(),
                depends: get_dependencies(config, Some(&Self::try_get(&map, "Depends").unwrap_or_default())),
                recommends: Self::split_optional(Some(&Self::try_get(&map, "Recommends").unwrap_or_default())),
                suggests: Self::split_optional(Some(&Self::try_get(&map, "Suggests").unwrap_or_default())),
                enhances: Self::split_optional(Some(&Self::try_get(&map, "Enhances").unwrap_or_default())),
                pre_depends: Self::split_optional(Some(&Self::try_get(&map, "Pre-Depends").unwrap_or_default())),
                breaks: Self::split_optional(Some(&Self::try_get(&map, "Breaks").unwrap_or_default())),
                conflicts: Self::split_optional(Some(&Self::try_get(&map, "Conflicts").unwrap_or_default())),
                conffiles: None,
                filename: Self::try_get(&map, "Filename").unwrap_or_default(),
                size: Self::try_get(&map, "Size").unwrap_or_default(),
                md5sum: Self::try_get(&map, "MD5sum").unwrap_or_default(),
                sha1: Self::try_get(&map, "SHA1").unwrap_or_default(),
                sha256: Self::try_get(&map, "SHA256").unwrap_or_default(),
                sha512: Self::try_get(&map, "SHA512").unwrap_or_default(),
            }
        )
    }

    // TODO: Maybe I need to make this easier to read
    fn try_get(hashmap: &HashMap<Option<String>, Option<String>>, field: &str) -> Result<String> {
        let value = hashmap.get(&Some(field.to_owned()));
        if let Some(v) = value {
            if let Some(v) = v {
                Ok (v.trim().to_owned())
            } else {
                bail!(ConfigError { msg: format!("Unknown error trying to get \"{}\" field", field) });
            }
        } else {
            bail!(ConfigError { msg: format!("Invalid debain's control file! Missing \"{}\" field", field) });
        }
    }

    fn split_optional(dependencies: Option<&str>) -> Option<Vec<String>> {
        if let Some(val) = dependencies {
            if !val.is_empty() {
                let val = val
                    .split(',')
                    .map(|d| d.trim().to_owned())
                    .collect::<Vec<_>>();
                Some(val)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_conffiles(conffiles: Option<&str>) -> Option<Vec<String>> {
        None
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
    pub fn new(config:&Config, file: &str, kind: PkgKind) -> Result<Self> {
        Ok(
            DebPackage {
                control: ControlFile::from_file(config, file)?,

                kind,
            }
        )
    }
}

#[cfg(test)]
mod test {
	use super::*;
    #[test]
    fn package_from_test() {
        let config = crate::repos::setup().unwrap();
        let data = r"Package: accountsservice
Architecture: amd64
Version: 0.6.55-0ubuntu11
Priority: standard
Section: gnome
Origin: Ubuntu
Maintainer: Ubuntu Developers <ubuntu-devel-discuss@lists.ubuntu.com>
Original-Maintainer: Debian freedesktop.org maintainers <pkg-freedesktop-maintainers@lists.alioth.debian.org>
Bugs: https://bugs.launchpad.net/ubuntu/+filebug
Installed-Size: 452
Depends: dbus, libaccountsservice0 (= 0.6.55-0ubuntu11), libc6 (>= 2.4), libglib2.0-0 (>= 2.44), libpolkit-gobject-1-0 (>= 0.99)
Suggests: gnome-control-center
Filename: pool/main/a/accountsservice/accountsservice_0.6.55-0ubuntu11_amd64.deb
Size: 60940
MD5sum: 87a0e27c83950d864d901ceca0f2b49c
SHA1: ce92ea3783ca4ca6cdb5115381379f9c1317566b
SHA256: e34884d71bb98002bf0c775479aa31ee5011ded1abf969ffe6496874de499f42
Homepage: https://www.freedesktop.org/wiki/Software/AccountsService/
Description: query and manipulate user account information
Task: standard
Description-md5: 8aeed0a03c7cd494f0c4b8d977483d7e";
		ControlFile::from(&config, data).unwrap();
	}

}