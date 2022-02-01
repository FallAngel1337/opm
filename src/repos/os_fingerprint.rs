use anyhow::Result;
use std::path::PathBuf;

use super::packages::PackageFormat;

///
/// Distro fingerprint files
///

const ISSUE: &str = "/etc/issue";
const ARCH_RELEASE: &str = "/etc/arch-release"; // Check if exists

///
/// Supported Distros
/// 

const DEBIAN: &str = "Debian";
const UBUNTU: &str = "Ubuntu";
const ARCH: &str = "Arch";
const OPENSUSE: &str = "Kernel"; // /etc/issue from opensuse is weird
const UNKNOWN: &str = "Unknown"; // /etc/issue from opensuse is weird

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
    Debian,
    Ubuntu,
    OpenSuse,
    Unknown,
}

// TODO: Add more architectures (https://wiki.debian.org/SupportedArchitectures)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Archs {
    Amd64,
    I386,
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OsInfo {
    pub os: OS,
    pub arch: Archs,
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
        use Distro::*;
        let issue = fs::read_to_string(ISSUE)?;
        let re = regex::Regex::new(r"\b(Debian|Ubuntu|Mint|Arch|Kernel)\b")?;

        if !re.is_match(&issue) {
            Ok(Unknown)
        } else {
            match re.captures(&issue).unwrap().get(0)
                .unwrap()
                .as_str()
            {
                DEBIAN => Ok(Debian),
                UBUNTU => Ok(Ubuntu),
                ARCH if Path::new(ARCH_RELEASE).exists() => Ok(Arch),
                OPENSUSE => Ok(OpenSuse),
                _ => Ok(Unknown),
            }
        }
    }
}


impl ToString for Distro {
    fn to_string(&self) -> String {
        match self {
            Self::Debian => DEBIAN.to_owned(),
            Self::Ubuntu => UBUNTU.to_owned(),
            Self::Arch => ARCH.to_owned(),
            Self::OpenSuse => OPENSUSE.to_owned(),
            _ => UNKNOWN.to_owned(),
        }
    }
}

impl Archs {
    #[cfg(target_arch="x86")]
    pub const fn get_arch() -> Self {
        Self::I386
    }

    #[cfg(target_arch="x86_64")]
    pub const fn get_arch() -> Self {
        Self::Amd64
    }

    #[cfg(not(any(target_arch="x86_64", target_arch="x86_64")))]
    pub const fn get_arch() -> Self {
        Self::Unknown
    }
}

impl ToString for Archs {
    fn to_string(&self) -> String {
        match self {
            Self::Amd64 => "amd64".to_owned(),
            Self::I386 => "i386".to_owned(),
            _ => "unknown".to_owned()
        }
    }
}

impl OsInfo {
    pub fn new() -> Result<Self> {
        let os = OS::get_os()?;
        let arch = Archs::get_arch();
        let previous_db = Self::get_db(&os);
        let default_package_format = Self::get_default_package_format(&os)?;
        let install_dir = Self::get_install_dir(&os).join(default_package_format.to_string());

        Ok(
            Self {
                os,
                arch,
                previous_db,
                default_package_format,
                install_dir
            }
        )
    }

    //TODO: Remove all the panics
    fn get_db(os: &OS) -> Option<PathBuf> {
        use OS::*;
        match os {
            Linux(distro) => {
                use Distro::*;
                match distro {
                    Arch => panic!("Using Arch ..."),
                    Debian | Ubuntu => {
                        use super::deb::database::DEBIAN_DATABASE;
                        Self::check_exists(DEBIAN_DATABASE)
                    },
                    OpenSuse => panic!("Using OpenSuse ..."),
                    Unknown => panic!("Using UNKNOWN ..."),
                }
            },
            Windows => panic!("Using windows"),
            Mac => panic!("Using Mac"),
            Unknown => panic!("Could not detect your OS"),
        }
    }

    fn get_default_package_format(os: &OS) -> Result<PackageFormat> {
        PackageFormat::get_format(os)
    }

    fn get_install_dir(os: &OS) -> PathBuf {
        use OS::*;
        match os {
            Linux(distro) => {
                use Distro::*; 
                match distro {
                    Arch | Debian | Ubuntu | OpenSuse => PathBuf::from(UNIX_INSTALL_DIR),
                    Unknown => panic!("Using UNKNOWN ..."),
                }
            },
            Windows => panic!("Using windows"),
            Mac => panic!("Using Mac"),
            Unknown => panic!("Could not detect your OS"),
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
