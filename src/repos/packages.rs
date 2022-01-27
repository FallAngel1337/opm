///
/// Package Formats
/// 

use anyhow::Result;
use super::os_fingerprint::{OS, Distro};

pub const DEB: &str = "deb";
pub const RPM: &str = "rpm";
pub const PKG: &str = "pkg";
pub const UNKNOWN: &str = "unknown"; // Unknown format (hope there's no `unk` package format)

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum PackageFormat {
    Deb,
    Rpm,
    Pkg,
    Unknown,
}

impl PackageFormat {
    pub fn get_format(os: &OS) -> Result<Self> {
        match os {
            OS::Linux(distro) => {
                match distro {
                    Distro::Arch => panic!("Using Arch ..."),
                    Distro::Debian => Ok(Self::Deb),
                    Distro::Rhel => Ok(Self::Rpm),
                    Distro::Unknown => Ok(Self::Unknown),
                }
            },
            OS::Windows => panic!("Using windows"),
            OS::Mac => panic!("Using Mac"),
            OS::Unknown => panic!("Could not detect your OS"),
        }
    }
}

impl ToString for PackageFormat {
    fn to_string(&self) -> String {
        match self {
            PackageFormat::Deb => DEB.to_owned(),
            PackageFormat::Rpm => RPM.to_owned(),
            PackageFormat::Pkg => PKG.to_owned(),
            _ => UNKNOWN.to_owned(),
        }
    }
}

