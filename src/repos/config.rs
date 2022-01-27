use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::io::ErrorKind;
use std::env;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	pub pkg_fmt: String,
	pub cache: String,
	pub rls: String,
	pub archive: String,
	pub info: String,
	pub tmp: String,
	pub db: String,

	pub use_pre_existing_cache: bool,
	pub use_pre_existing_db: bool
}

#[allow(deprecated)]
impl Config {
	pub fn new(pkg_fmt: &str) -> Result<Self> {
		let home = env::home_dir().unwrap()
			.into_os_string().into_string().unwrap();
		let dir = format!("{}/.opm/{}", home, pkg_fmt);
		
		Ok(
			Self {
				cache: format!("{}/cache/pkg", dir),
				rls: format!("{}/cache/rls", dir),
				tmp: format!("{}/tmp", dir),
				archive: format!("{}/archive", dir),
				info: format!("{}/info", dir),
				db: format!("{}/db", dir),
				pkg_fmt: pkg_fmt.to_owned(),
				use_pre_existing_cache: false,
				use_pre_existing_db: false,
			}
		)
	}

	pub fn from(file: &str) -> Self {
		let contents = fs::read_to_string(file).unwrap();
		toml::from_str(&contents).unwrap()
	}

	pub fn save(&self, to: &str) {
		let contents = toml::to_string(self).unwrap();
		fs::write(to, contents).unwrap();
	}

	pub fn setup(&self) -> Result<()> {
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

		match fs::create_dir_all(&self.archive) {
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

		match fs::File::create(&self.db) {
			Ok(_) => {
				use super::os_fingerprint::PackageFormat;
				match PackageFormat::from(&self.pkg_fmt) {
					PackageFormat::Deb => {
						use super::deb::database::DEBIAN_DATABASE;
						if let Err(err) = fs::copy(DEBIAN_DATABASE, &self.db) {
							if err.kind() != std::io::ErrorKind::NotFound {
								anyhow::bail!(err);
							} else {
								fs::File::create(&self.db)?;
							}
						}
					},
					PackageFormat::Rpm => panic!("We do not support RPM packages for now ..."),
					PackageFormat::Other => panic!("Unrecognized package"),
				}
			},
			Err(e) => match e.kind() {
			ErrorKind::AlreadyExists => (),
				_ => panic!("Some error occurred {}", e)
			}
		}

		Ok(())
	}
}