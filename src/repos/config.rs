use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::io::ErrorKind;
use std::env;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	pub root: String,
	pub cache: String,
	pub info: String,
	pub rls: String,
	pub tmp: String,
	pub fmt: String,

	pub use_pre_existing_cache: bool,
	pub use_pre_existing_db: bool
}

#[allow(deprecated)]
impl Config {
	pub fn new(fmt: &str) -> Result<Self> {
		let home = env::home_dir().unwrap()
			.into_os_string().into_string().unwrap();
		let root = format!("{}/.opm/{}", home, fmt);
		Ok(
			Self {
				cache: format!("{}/cache/pkg_cache", root),
				rls: format!("{}/cache/rls", root),
				tmp: format!("{}/tmp", root),
				info: format!("{}/info", root),
				fmt: fmt.to_owned(),
				use_pre_existing_cache: false,
				use_pre_existing_db: false,
				root
			}
		)
	}

	pub fn from(file: &str) -> Self {
		let contents = fs::read_to_string(file).unwrap();
		serde_json::from_str(&contents).unwrap()
	}

	pub fn save(&self, to: &str) {
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