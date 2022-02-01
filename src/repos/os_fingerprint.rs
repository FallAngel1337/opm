use anyhow::Result;
use std::path::PathBuf;
use os_release::OsRelease;

use super::packages::PackageFormat;

///
/// Supported Distros
/// 

const DEBIAN: &str = "Debian";
const UBUNTU: &str = "Ubuntu";
const ARCH: &str = "Arch";
const OPENSUSE: &str = "openSUSE"; // /etc/issue from opensuse is weird
const UNKNOWN: &str = "Unknown"; // /etc/issue from opensuse is weird

///
/// Default Installation dir
///

const UNIX_INSTALL_DIR: &str = "/opt/opm/";
// const WIN_INSTALL_DIR: &str = "C:\\OPM";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Os {
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
    pub os: Os,
    pub arch: Archs,
    pub version: String,
    pub previous_db: Option<PathBuf>,
    pub default_package_format: PackageFormat,
    pub install_dir: PathBuf,
}

impl Os {
    fn new() -> Result<Os> {
        if cfg!(linux) || cfg!(unix) {
            Ok(Os::Linux(Distro::new(&OsRelease::new()?.name)?))
        } else if cfg!(macos) {
            Ok(Os::Mac)
        } else if cfg!(windows) {
            Ok(Os::Windows)
        } else {
            Ok(Os::Unknown)
        }
    }
}

impl Distro {
    fn new(name: &str) -> Result<Self> {
        use std::path::Path;
        use Distro::*;

        let name = name.split_whitespace().next().unwrap();
        let re = regex::Regex::new(r"\b(Debian|Ubuntu|Arch|openSuse)\b")?;

        if !re.is_match(name) {
            Ok(Unknown)
        } else {
            match re.captures(name).unwrap().get(0)
                .unwrap()
                .as_str()
            {
                DEBIAN => Ok(Debian),
                UBUNTU => Ok(Ubuntu),
                ARCH if Path::new("/etc/arch-release").exists() => Ok(Arch),
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
    pub const fn new() -> Self {
        Self::I386
    }

    #[cfg(target_arch="x86_64")]
    pub const fn new() -> Self {
        Self::Amd64
    }

    #[cfg(not(any(target_arch="x86_64", target_arch="x86_64")))]
    pub const fn new() -> Self {
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
        let os = Os::new()?;
        let arch = Archs::new();
        let version = Self::version(&os)?;
        let previous_db = Self::db(&os);
        let default_package_format = Self::default_package_format(&os)?;
        let install_dir = Self::install_dir(&os).join(default_package_format.to_string());

        Ok(
            Self {
                os,
                arch,
                version,
                previous_db,
                default_package_format,
                install_dir
            }
        )
    }

    //TODO: Remove all the panics
    fn db(os: &Os) -> Option<PathBuf> {
        use Os::*;
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
            Unknown => panic!("Could not detect your Os"),
        }
    }

    fn default_package_format(os: &Os) -> Result<PackageFormat> {
        PackageFormat::format(os)
    }

    fn install_dir(os: &Os) -> PathBuf {
        use Os::*;
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
            Unknown => panic!("Could not detect your Os"),
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

    fn version(os: &Os) -> Result<String> {
        use Os::*;
        match os {
            Linux(_) => Ok(OsRelease::new()?.version_id),
            Windows => panic!("Using windows"),
            Mac => panic!("Using Mac"),
            Unknown => panic!("Could not detect your Os"),
        }
    }
}
