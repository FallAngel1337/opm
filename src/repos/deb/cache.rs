use crate::repos::config::Config;
use super::{
	package::{ControlFile, DebPackage, PkgKind}
};
use std::{fs, io::ErrorKind};

const DEBIAN_CACHE: &str = "/var/lib/apt/lists/";

struct Cache<'a> {
	cache: &'a str
}

impl<'a> Cache<'a> {
	fn get_cache(config: &'a Config) -> Self {
		if config.use_pre_existing_cache {
			Cache {
				cache: DEBIAN_CACHE
			}
		} else {
			Cache {
				cache: &config.cache
			}
		}
	}
}

///
/// Dump from the downloded files
///
pub fn cache_dump(config: &Config) -> Vec<ControlFile> {
	let mut pkgs = Vec::new();
	let cache = Cache::get_cache(config);
	
	for entry in fs::read_dir(cache.cache).unwrap() {
		let entry = entry.unwrap();
		let path = entry.path();

		if path.is_dir() {
			continue
		}

		let control = match fs::read_to_string(&path) {
			Ok(v) => v,
			Err(e) => match e.kind() {
				ErrorKind::PermissionDenied => continue,
				_ => panic!("Unexpected error")
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

	pkgs
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

pub fn cache_lookup(config: &Config, name: &str) -> Option<DebPackage> {
	let mut pkgs = Vec::new();
	let cache = Cache::get_cache(config);

	for entry in fs::read_dir(cache.cache).unwrap() {
		let entry = entry.unwrap();
		let path = entry.path();
		
		let control = fs::read_to_string(path).unwrap();

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

		for ctrl in control.split("\n\n") {
			for line in ctrl.split('\n') {
				if line.contains("Package: ") {
					let pkg = line.split(": ").nth(1).unwrap();
					if pkg == name {
						let mut control_file = ControlFile::from(ctrl).unwrap();
						let url = format!("{}/{}", url, &control_file.filename);
						control_file.set_filename(&url);
						pkgs.push(control_file);
					}
				}
			}
		}
	}

	pkgs.into_iter().map(|control| DebPackage { control, kind: PkgKind::Binary }).next()
}

#[inline]
pub fn check_installed(name: &str) -> Option<DebPackage> {
	dump_installed().into_iter().find(|pkg| pkg.control.package == name)
}

#[allow(unused)]
pub fn add_package(config: &Config, pkg: DebPackage, cache: bool) -> Result<(), std::io::Error> {
	Ok(())
}