use anyhow::{self, Result, Context};
use crate::repos::{config::Config, errors::InstallError};
use crate::repos::errors::CacheError;
use std::fs;

use super::{
	package::{ControlFile, DebPackage, PkgKind}
};

const DEBIAN_CACHE: &str = "/var/lib/apt/lists/";
struct Cache<'a> {
	cache: &'a str
}

impl<'a> Cache<'a> {
	fn get_cache(config: &'a Config) -> Result<Self> {
		if config.use_pre_existing_cache {
			if !std::path::Path::new(DEBIAN_CACHE).exists() {
				anyhow::bail!(CacheError { msg: format!("{} was not found", DEBIAN_CACHE) });
			}
			
			Ok(
				Cache {
					cache: DEBIAN_CACHE
				}
			)
		} else {
			if !std::path::Path::new(&config.cache).exists() {
				anyhow::bail!(CacheError { msg: format!("{} was not found", config.cache) });
			}

			Ok(
				Cache {
					cache: &config.cache
				}
			)
		}
	}
}

///
/// Dump from the downloded files
///
pub fn cache_dump(config: &Config) -> Result<Vec<ControlFile>> {
	let mut pkgs = Vec::new();
	let cache = Cache::get_cache(config)
		.context("Failed to read the cache file")?;
	
	for entry in fs::read_dir(cache.cache)? {
		let entry = entry.unwrap();
		let path = entry.path();
		let path_str = path.clone().into_os_string().into_string().unwrap();

		if path.is_dir() || !path_str.contains('_') {
			continue
		}

		let control = match fs::read_to_string(&path) {
			Ok(v) => v,
			Err(e) => {
				eprintln!("Unexpected error :: {}", e);
				break;
			}
		};

		let control = control
			.split("\n\n")
			.map(ControlFile::from);
			
		let entry = entry.path()
			.into_os_string()
			.into_string()
			.unwrap();

		let url =  &entry
			.split('/')
			.last()
			.unwrap()
			.replace("_", "/")
			.split('/')
			.collect::<Vec<_>>()[..2]
			.join("/");

		control.into_iter().filter_map(|pkg| pkg.ok()).for_each(|mut pkg| {
			let url = format!("{}/{}", url, &pkg.filename);
			pkg.set_filename(&url);
			pkgs.push(pkg);
		});
	}

	Ok(pkgs)
}

///
/// Dump all installed packages from /var/lib/dpkg/status
/// 
pub fn dump_installed() -> Vec<DebPackage> {
	let control = fs::read_to_string(super::database::DEBIAN_DATABASE).unwrap();

	let control = control
		.split("\n\n")
		.map(ControlFile::from)
		.filter_map(|ctrl| ctrl.ok())
		.map(|ctrl| DebPackage { control: ctrl, kind: PkgKind::Binary } )
		.collect::<Vec<_>>();
	
	control
}

pub fn cache_lookup(config: &Config, name: &str) -> Result<Option<DebPackage>> {
	let dump = cache_dump(config)?
		.into_iter()
		.find(|control| control.package == name);

	if let Some(control) = dump {
		Ok(
			Some (
				DebPackage {
					control,
					kind: PkgKind::Binary
				}
			)
		)
	} else {
		anyhow::bail!(InstallError::NotFoundError(format!("Package {} coult not be found (at {})", name, config.cache)))
	}
}

#[inline]
pub fn check_installed(name: &str) -> Option<DebPackage> {
	dump_installed().into_iter().find(|pkg| pkg.control.package == name)
}

#[allow(unused)]
pub fn add_package(config: &Config, pkg: DebPackage, cache: bool) -> Result<(), std::io::Error> {
	Ok(())
}