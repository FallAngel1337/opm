use anyhow::{self, Result, Context};
use crate::repos::{
	config::Config,
	errors::CacheError,
};
use std::{fs, io::prelude::*};
use regex::Regex;

use super::package::{ControlFile, DebPackage, PkgKind};

const DEBIAN_CACHE: &str = "/var/lib/apt/lists/";

#[derive(Debug)]
struct Cache<'a> {
	cache: &'a str
}

#[derive(Debug)]
struct CacheResult {
	pkg: Option<DebPackage>,
	pkgs: Option<Vec<DebPackage>>
}

impl<'a> Cache<'a> {
	fn get_cache(config: &'a Config) -> Result<Self> {
		if config.use_pre_existing_cache {
			if !std::path::Path::new(DEBIAN_CACHE).exists() {
				anyhow::bail!(CacheError::NoCache(DEBIAN_CACHE.to_owned()))
			}
			
			Ok(
				Cache {
					cache: DEBIAN_CACHE
				}
			)
		} else {
			if !std::path::Path::new(&config.cache).exists() {
				anyhow::bail!(CacheError::NoCache(config.cache.to_owned()))
			}

			Ok(
				Cache {
					cache: &config.cache
				}
			)
		}
	}
}

pub fn db_dump(config: &Config) -> Vec<DebPackage> {
	let db = if config.use_pre_existing_db {
		super::database::DEBIAN_DATABASE
	} else {
		&config.db
	};
	
	let control = fs::read_to_string(db).unwrap();

	let control = control
		.split("\n\n")
		.map(|contents| ControlFile::new(config, contents))
		.filter_map(|ctrl| ctrl.ok())
		.map(|ctrl| DebPackage { control: ctrl, kind: PkgKind::Binary } )
		.collect::<Vec<_>>();
	
	control
}

fn cache_inter(config: &Config, name: &str, exact: bool) -> Result<CacheResult> {
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

		let mut control = control
		.split("\n\n")
		.map(|contents| ControlFile::new(config, contents))
		.filter_map(|pkg| pkg.ok());

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

		if exact {
			let control = control.find(|pkg| pkg.package == name);

			if let Some(mut pkg) = control {
				let url = format!("{}/{}", url, &pkg.filename);
				pkg.set_filename(&url);
				return Ok(
					CacheResult {
						pkg: Some(
							DebPackage {
								control: pkg,
								kind: PkgKind::Binary
							}
						),
						pkgs: None
					}
				)
			}
		} else {
			let re = Regex::new(name)?;

			let pkgs = control
				.filter(|pkg| re.is_match(&pkg.package))
				.map(|mut pkg| {
					let url = format!("{}/{}", url, &pkg.filename);
					pkg.set_filename(&url);
					DebPackage {
						control: pkg,
						kind: PkgKind::Binary
					}
				})
				.collect::<Vec<_>>();

			if !pkgs.is_empty() {
				return Ok(
					CacheResult {
						pkg: None,
						pkgs: Some(pkgs)
					}
				);
			}
		}
	}

	anyhow::bail!(CacheError::NotFoundError { pkg: name.to_string(), cache: config.cache.clone() });
}

///
/// Search for a package in the cache that `contains` `name`
/// 
#[inline]
pub fn cache_search(config: &Config, name: &str) -> Result<Option<Vec<DebPackage>>> {
	Ok (
		cache_inter(config, name, false)?.pkgs
	)
}

///
/// Search for a package in the cache that is equal to `name`
/// 
#[inline]
pub fn cache_lookup(config: &Config, name: &str) -> Result<Option<DebPackage>> {
	Ok (
		cache_inter(config, name, true)?.pkg
	)
}

#[inline]
pub fn check_installed(config: &Config, name: &str) -> Option<DebPackage> {
	db_dump(config).into_iter().find(|pkg| pkg.control.package == name)
}

pub fn add_package(config: &Config, pkg: DebPackage) -> Result<()> {
	let pkg = pkg.control;
	let db = if config.use_pre_existing_db {
		super::database::DEBIAN_DATABASE
	} else {
		&config.db
	};

	let mut data = format!("Package: {}
Version: {}
Priority: {}
Architecture: {}
Maintainer: {}
Description: {}", pkg.package, pkg.version, pkg.priority, pkg.architecture, pkg.maintainer, pkg.description);

	if let Some(d) = pkg.depends {
		let mut depends = String::new();
		d.into_iter().for_each(|pkg| depends.push_str(&pkg));
		data.push_str(&format!("\nDepends: {}", depends));
	}

	if let Some(d) = pkg.breaks {
		let breaks = d.join(", ");
		data.push_str(&format!("\nBreaks: {}", breaks));
	}
	
	if let Some(d) = pkg.conflicts {
		let conflicts = d.join(", ");
		data.push_str(&format!("\nConflicts: {}", conflicts));
	}

	data.push('\n');

	let mut file = fs::OpenOptions::new()
		.write(true)
		.append(true)
		.open(db)?;

	if let Err(e) = writeln!(file, "{}", data) {
		eprintln!("Couldn't write to db: {}", e);
	}

	Ok(())
}

pub fn rm_package(config: &Config, name: &str) -> Result<()> {
	let db = if config.use_pre_existing_db {
		super::database::DEBIAN_DATABASE
	} else {
		&config.db
	};

	let data = fs::read_to_string(&db)?;
	let mut db = fs::File::create(&db)?;

	if let Some(index) = data.find(&format!("Package: {}", name)) {
		db.write_all(data[..index].as_ref())?;
	}

	Ok(())
}

#[cfg(test)]
mod test {
	use crate::repos;
	use super::*;

	#[test]
	fn get_cache_test() {
		let config = repos::setup().unwrap();
		dbg!("[get_cache_test]", &config);
		Cache::get_cache(&config).unwrap();
	}

	#[test]
	#[should_panic]
	fn cache_search_test() {
		let config = repos::setup().unwrap();
		let _pkg = cache_search(&config, "invalidPackage0101").unwrap();
	}

	#[test]
	fn db_dump_test() {
		let config = repos::setup().unwrap();
		// THIS MAY NOT BE GOOD, IF YOU HAVE AN EMPTY DATABASED IT'LL FAIL
		dbg!("[db_dump_test]", &config);
		assert!(db_dump(&config).len() > 0);
	}

	// This was crashing and idk why
	#[test]
	#[should_panic]
	fn cache_lookup_test() {
		let config = repos::setup().unwrap();
		dbg!("[cache_lookup_test]", &config);
		let pkg = cache_lookup(&config, "invalidPackage0101").unwrap();
		assert!(pkg.is_none());
	}
}