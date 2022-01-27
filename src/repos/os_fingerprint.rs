use anyhow::Result;
use std::path::PathBuf;

use super::packages::PackageFormat;

///
/// Distro fingerprint files
///

const DEBIAN: &str = "/etc/issue";      // Check if have "Debian GNU/Linux"
const ARCH: &str = "/etc/arch-release"; // Check if exists

///
/// Default Installation dir
///

const UNIX_INSTALL_DIR: &str = "/opt/opm/";
// const WIN_INSTALL_DIR: &str = "C:\\OPM";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum OS {
    Windows,
    Linux(Distro),
    Mac,
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Distro {
    Arch,
    Debian, // Basically all debian-based
    Rhel,
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OsInfo {
    pub os: OS,
    pub previous_db: Option<PathBuf>,
    pub default_package_format: PackageFormat,
    pub install_dir: PathBuf,
}

impl OS {
    fn get_os() -> Result<OS> {
        if cfg!(linux) || cfg!(unix) {
            Ok(OS::Linux(Distro::get_distro()?))
        } else if cfg!(macos) {
            Ok(OS::Mac)
        } else if cfg!(windows) {
            Ok(OS::Windows)
        } else {
            Ok(OS::Unknown)
        }
    }
}

impl Distro {
    fn get_distro() -> Result<Self> {
        use std::{path::Path, fs};
        use Distro::{Arch, Debian, Rhel, Unknown};
        let data = fs::read_to_string(DEBIAN)?;
        match ((Path::new(ARCH)).exists(), (data.contains("Debian") || data.contains("Ubuntu"))) {
            (true, false) => Ok(Arch),
            (false, true) => Ok(Debian),
            (false, false) => Ok(Rhel), // TODO: Find a way of detecting this
            _ => Ok(Unknown)
        }
    }
}

impl OsInfo {
    pub fn new() -> Result<Self> {
        let os = OS::get_os()?;
        let previous_db = Self::get_db(&os);
        let default_package_format = Self::get_default_package_format(&os)?;
        let install_dir = Self::get_install_dir(&os).join(default_package_format.to_string());

        Ok(
            Self {
                os,
                previous_db,
                default_package_format,
                install_dir
            }
        )
    }

    //TODO: Remove all the panics
    fn get_db(os: &OS) -> Option<PathBuf> {
        match os {
            OS::Linux(distro) => {
                match distro {
                    Distro::Arch => panic!("Using Arch ..."),
                    Distro::Debian => {
                        use super::deb::database::DEBIAN_DATABASE;
                        Self::check_exists(DEBIAN_DATABASE)
                    },
                    Distro::Rhel => panic!("Using RHEL ..."),
                    Distro::Unknown => panic!("Using UNKNOWN ..."),
                }
            },
            OS::Windows => panic!("Using windows"),
            OS::Mac => panic!("Using Mac"),
            OS::Unknown => panic!("Could not detect your OS"),
        }
    }

    fn get_default_package_format(os: &OS) -> Result<PackageFormat> {
        PackageFormat::get_format(os)
    }

    fn get_install_dir(os: &OS) -> PathBuf {
        match os {
            OS::Linux(distro) => {
                match distro {
                    Distro::Arch | Distro::Debian | Distro::Rhel => PathBuf::from(UNIX_INSTALL_DIR),
                    Distro::Unknown => panic!("Using UNKNOWN ..."),
                }
            },
            OS::Windows => panic!("Using windows"),
            OS::Mac => panic!("Using Mac"),
            OS::Unknown => panic!("Could not detect your OS"),
        }
    }

    fn check_exists(path: &str) -> Option<PathBuf> {
        let db = PathBuf::from(path);
        if db.exists() {
            Some(db)
        } else {
            None
        }
    }
}
