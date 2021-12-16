use std::path::PathBuf;
use std::io::{Error, ErrorKind};
use std::env;
use std::fs;

use super::{database::SQLite, errors::ConfigError};
use super::utils::PackageFormat;

#[derive(Debug)]
pub struct Config {
	pub root: PathBuf,
	pub cache: PathBuf,
	pub db: PathBuf,
	pub rls: PathBuf,
	pub tmp: PathBuf,

	pub sqlite: Option<SQLite>,
}

#[allow(deprecated)]
impl Config {
	pub fn new() -> Result<Self, ConfigError> {
		let home = env::home_dir().unwrap()
			.into_os_string().into_string().unwrap();
		let pkg_fmt_name;
			
		if let Some(pkg_fmt) = PackageFormat::get_format() {
			pkg_fmt_name = match pkg_fmt {
				PackageFormat::Deb => {
					"deb"
				},
				PackageFormat::Rpm => {
					println!("It's a RHEL(-based) distro");
					"rpm"
				},
				PackageFormat::Other => {
					println!("Actually we do not have support for you distro!");
					"oth"
				},
			};
		} else {
			eprintln!("Consider define `PKG_FMT` environment variable!");
			std::process::exit(1);
		}

		let opm_root = format!("{}/.opm/{}", home, pkg_fmt_name);

		Ok(
			Self {
				root: PathBuf::from(&opm_root),
				cache: PathBuf::from(format!("{}/cache/pkg_cache", opm_root)),
				db: PathBuf::from(format!("{}/db/pkgs.db", opm_root)),
				rls: PathBuf::from(format!("{}/cache/rls", opm_root)),
				tmp: PathBuf::from(format!("{}/tmp", opm_root)),
				sqlite: None
			}
		)
	}

	pub fn close_db(&self) {
		if let Some(conn) = self.sqlite.as_ref() {
			conn.close()
		}
	}

	pub fn setup_db(&mut self) -> rusqlite::Result<()> {
		if self.sqlite.as_ref().is_none() {
			self.sqlite = Some(
				SQLite::new(&self.db)?
			);
		}
		Ok(())
	}

	pub fn setup(&mut self) -> Result<(), Error> {
		let path = std::path::Path::new(&self.db);
		let prefix = path.parent().unwrap();
		fs::create_dir_all(prefix)?;

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

		Ok(())
	}
}