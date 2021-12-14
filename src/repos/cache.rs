use crate::repos::deb::package::DebPackage;

use super::utils::PackageFormat;
use super::config::Config;
use rusqlite::Result;
use std::fs;

///
/// Lookup into the local cache(~/.opm/cache)
/// 
// TODO: Improve it to be less slow
use super::deb::{package::ControlFile};
pub fn cache_lookup(config: &Config, name: &str, exact_match: bool) -> Option<Vec<ControlFile>> {
	let mut pkgs = Vec::new();
	for entry in fs::read_dir(&config.cache).unwrap() {
		let entry = entry.unwrap();
		let path = entry.path();
		
		let control = fs::read_to_string(path).unwrap();

		let control = control
			.split("\n\n")
			.map(|ctrl| ControlFile::from(config, ctrl));
			
		let control = if exact_match {
			control
				.filter(|ctrl| ctrl.package == name)
				.collect::<Vec<_>>()
		} else {
			control
				.filter(|ctrl| ctrl.package.contains(name))
				.collect::<Vec<_>>()
		};

		let entry = entry.path()
			.into_os_string()
			.into_string()
			.unwrap();

		let url =  entry
			.split("/")
			.last()
			.unwrap()
			.replace("_", "/")
			.split("/")
			.next()
			.unwrap()
			.to_owned();

		control.into_iter().for_each(|mut pkg| {
			let url = format!("{}/ubuntu/{}", url, &pkg.filename);
			pkg.set_filename(&url);
			pkgs.push(pkg);
		});
	}

	if pkgs.len() > 0 {
		Some(pkgs)
	} else {
		None
	}
}

pub fn list_installed(config: &Config) {
	// config.setup_db();

	if let Some(pkg_fmt) = PackageFormat::get_format() {
		match pkg_fmt {
			PackageFormat::Deb => {
				
				if let Some(sqlite) = config.sqlite.as_ref() {
					if let Ok(pkg) = sqlite.pkg_list() {
						pkg.into_iter().for_each(|pkg| {
							println!("{:?}", pkg);
						})
					}
				}
			}
			PackageFormat::Rpm => {
				println!("It's a RHEL(-based) distro");
				}
				PackageFormat::Other => {
					println!("Actually we do not have support for you distro!");
				}
			}
	} else {
        eprintln!("Consider define `PKG_FMT` environment variable!");
        std::process::exit(1);
    }
}

pub fn search(config: &Config, name: &str) {
    if let Some(pkg_fmt) = PackageFormat::get_format() {
		match pkg_fmt {
			PackageFormat::Deb => {
				if let Some(sqlite) = config.sqlite.as_ref() {
					println!("Found:");
					if let Ok(pkg) = sqlite.lookup(name, true) {
						if let Some(pkg) = pkg {
							println!("{:?}", pkg);
						}
					}
				}
			}
			PackageFormat::Rpm => {
				println!("It's a RHEL(-based) distro");
			}
			PackageFormat::Other => {
				println!("Actually we do not have support for you distro!");
			}
		}
    } else {
        eprintln!("Consider define `PKG_FMT` environment variable!");
        std::process::exit(1);
	}
}

pub fn dump_into_db(config: &mut Config) -> Result<()> {
	if let Some(pkg_fmt) = PackageFormat::get_format() {
		match pkg_fmt {
			PackageFormat::Deb => {
				use super::deb::{cache, package::PkgKind};
				let pkgs = cache::dpkg_cache_dump(&config);
				println!("Detected a dpkg database (assuming it's debian)");
				for pkg in pkgs.into_iter()  {
					let deb_pkg = DebPackage {
						control: pkg,
						kind: PkgKind::Binary,
        				signature: "NOPE".to_owned()
					};
					let sqlite = config.sqlite.as_ref().unwrap();
					sqlite.add_package(deb_pkg)?;
				};
				
				Ok(())
			}
			PackageFormat::Rpm => {
				println!("It's a RHEL(-based) distro");
				Ok(())
			}
			PackageFormat::Other => {
				println!("Actually we do not have support for you distro!");
				Ok(())
			}
		}
    } else {
        eprintln!("Consider define `PKG_FMT` environment variable!");
        std::process::exit(1);
	}
}
