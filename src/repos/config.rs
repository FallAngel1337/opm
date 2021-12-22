use serde::{Serialize, Deserialize};
use std::io::{Error, ErrorKind};
use std::env;
use std::fs;

use super::{errors::ConfigError, utils::PackageFormat};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	pub root: String,
	pub cache: String,
	pub info: String,
	pub rls: String,
	pub tmp: String,

	pub use_pre_existing_cache: bool,
	pub use_pre_existing_db: bool
}

#[allow(deprecated)]
impl Config {
	pub fn new(pkg_fmt: PackageFormat) -> Result<Self, ConfigError> {
		let home = env::home_dir().unwrap()
			.into_os_string().into_string().unwrap();
		let opm_root;

        match pkg_fmt {
            PackageFormat::Deb => {
				opm_root = format!("{}/.opm/{}", home, "deb");
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
				cache: format!("{}/cache/pkg_cache", opm_root),
				rls: format!("{}/cache/rls", opm_root),
				tmp: format!("{}/tmp", opm_root),
				info: format!("{}/info", opm_root),
				root: opm_root,
				use_pre_existing_cache: false,
				use_pre_existing_db: false,
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