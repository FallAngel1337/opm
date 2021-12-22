use std::path::PathBuf;
use std::io::{Error, ErrorKind};
use std::env;
use std::fs;

use super::{errors::ConfigError, utils::PackageFormat};

#[derive(Debug)]
pub struct Config {
	pub root: PathBuf,
	pub cache: PathBuf,
	pub info: PathBuf,
	pub rls: PathBuf,
	pub tmp: PathBuf,
}

#[allow(deprecated)]
impl Config {
	pub fn new(pkg_fmt: PackageFormat) -> Result<Self, ConfigError> {
		let home = env::home_dir().unwrap()
			.into_os_string().into_string().unwrap();
		let (info, opm_root);

        match pkg_fmt {
            PackageFormat::Deb => {
                // use super::deb;
				opm_root = format!("{}/.opm/{}", home, "deb");
                info = format!("{}/info", opm_root); /*deb::database::DPKG_STATUS;*/
            },
            PackageFormat::Rpm => {
                panic!("It's a RHEL(-based) distro");
            },
            PackageFormat::Other => {
                panic!("Actually we do not have support for you distro!");
            }
        }


		Ok(
			Self {
				root: PathBuf::from(&opm_root),
				cache: PathBuf::from(format!("{}/cache/pkg_cache", opm_root)),
				info: PathBuf::from(info),
				rls: PathBuf::from(format!("{}/cache/rls", opm_root)),
				tmp: PathBuf::from(format!("{}/tmp", opm_root)),
			}
		)
	}

	pub fn setup(&mut self) -> Result<(), Error> {
		match fs::create_dir_all(&self.cache) {
			Ok(_) => (),
			Err(e) => match e.kind() {
			ErrorKind::AlreadyExists => (),
				_ => panic!("Some error occurred {}", e)
			}
		}

		match fs::create_dir_all(&self.rls) {
			Ok(_) => (),
			Err(e) => match e.kind() {
			ErrorKind::AlreadyExists => (),
				_ => panic!("Some error occurred {}", e)
			}
		}

		match fs::create_dir_all(&self.tmp) {
			Ok(_) => (),
			Err(e) => match e.kind() {
			ErrorKind::AlreadyExists => (),
				_ => panic!("Some error occurred {}", e)
			}
		}

		match fs::create_dir_all(&self.info) {
			Ok(_) => (),
			Err(e) => match e.kind() {
			ErrorKind::AlreadyExists => (),
				_ => panic!("Some error occurred {}", e)
			}
		}

		Ok(())
	}
}