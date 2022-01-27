use anyhow::Result;
use std::io::ErrorKind;
use std::fs;

use super::os_fingerprint::OsInfo;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
	pub os_info: OsInfo,
	pub cache: String,
	pub rls: String,
	pub archive: String,
	pub info: String,
	pub tmp: String,
	pub db: String,

	pub use_pre_existing_cache: bool,
	pub use_pre_existing_db: bool
}

impl Config {
	pub fn new(os_info: &OsInfo) -> Result<Self> {
		let dir = os_info.install_dir.clone();
		Ok(
			Self {
				os_info: os_info.clone(),
				cache: dir.join("cache/pkg").to_str().unwrap().to_owned(),
				rls: dir.join("cache/rls").to_str().unwrap().to_owned(),
				tmp: dir.join("tmp").to_str().unwrap().to_owned(),
				archive: dir.join("archibe").to_str().unwrap().to_owned(),
				info: dir.join("info").to_str().unwrap().to_owned(),
				db: dir.join("db").to_str().unwrap().to_owned(),
				use_pre_existing_cache: false,
				use_pre_existing_db: false,
			}
		)
	}

	pub fn from<P: AsRef<std::path::Path>>(file: P) -> Self {
		let contents = fs::read_to_string(file).unwrap();
		serde_json::from_str(&contents).unwrap()
	}

	pub fn save<P: AsRef<std::path::Path>>(&self, to: P) {
		let contents = serde_json::to_string(self).unwrap();
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
				if let Some(db) = self.os_info.previous_db.clone() {
					fs::copy(db, &self.db)?;
				} else {
					fs::File::create(&self.db)?;
				}
				// match &self.pkg_fmt {
				// 	PackageFormat::Deb => {
				// 		use super::deb::database::DEBIAN_DATABASE;
				// 		if let Err(err) = fs::copy(DEBIAN_DATABASE, &self.db) {
				// 			if err.kind() != std::io::ErrorKind::NotFound {
				// 				anyhow::bail!(err);
				// 			} else {
				// 				fs::File::create(&self.db)?;
				// 			}
				// 		}
				// 	},
				// 	PackageFormat::Rpm => panic!("We do not support RPM packages for now ..."),
				// 	PackageFormat::Rpm => panic!("We do not support RPM packages for now ..."),
				// 	PackageFormat::Unknown => panic!("Unrecognized package"),
				// }
			},
			Err(e) => match e.kind() {
			ErrorKind::AlreadyExists => (),
				_ => panic!("Some error occurred {}", e)
			}
		}

		Ok(())
	}
}