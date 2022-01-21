use core::panic;

use anyhow::Result;

const DEBIAN: &str = "/etc/issue";      // Check if have "Debian GNU/Linux"
const ARCH: &str = "/etc/arch-release"; // Check if exists

#[derive(Debug)]
enum OS {
    Windows,
    Linux(Distro),
    Mac,
    Unknown,
}

#[derive(Debug)]
enum Distro {
    Arch,
    Debian, // Basically all debian-based
    Rhel,
    Unknown,
}

#[derive(Debug)]
pub enum PackageFormat {
    Deb,
    Rpm,
    Other,
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
            (false, false) => Ok(Rhel), // Find a way of detecting this
            _ => Ok(Unknown)
        }
    }
}

impl PackageFormat {
    pub fn get_format() -> Result<Self> {
        use OS::{Linux, Windows, Mac, Unknown};
        match OS::get_os()? {
            Linux(distro) => {
                use Distro::{Arch, Debian, Rhel, Unknown};
                match distro {
                    Arch => panic!("Using Arch ..."),
                    Debian => Ok(Self::Deb),
                    Rhel => Ok(Self::Rpm),
                    Unknown => Ok(Self::Other),
                }
            },
            Windows => panic!("Using windows"),
            Mac => panic!("Using Mac"),
            Unknown => panic!("Could not detect your OS"),
        }
    }

    pub fn from(fmt: &str) -> Self {
        match fmt {
            "deb" => PackageFormat::Deb,
            "rpm" => PackageFormat::Rpm,
            "oth" => PackageFormat::Other,
            _ => panic!("Invalid format") // TODO: Raise a custom error
        }
    }
}
