use crate::repos::{deb::package::DebPackage, database::PackageStatus};

use super::{utils::PackageFormat, database::GenericPackage};
use super::config::Config;
use rusqlite::Result;

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

pub fn lookup(config: &Config, name: &str) -> Option<Vec<GenericPackage>> {
	if let Some(sqlite) = config.sqlite.as_ref() {
		if let Ok(pkgs) = sqlite.lookup(name, true) {
			if let Some(pkgs) = pkgs {
				if pkgs.len() > 0 {
					Some(pkgs)
				} else {
					None
				}
			} else {
				None
			}
		} else {
			panic!("Database query failed");
		}
	} else {
		panic!("Something gone wrong")
	}
}

pub fn search(config: &Config, name: &str) {
    if let Some(pkg_fmt) = PackageFormat::get_format() {
		match pkg_fmt {
			PackageFormat::Deb => {
				if let Some(sqlite) = config.sqlite.as_ref() {
					println!("Found:");
					if let Ok(pkg) = sqlite.lookup(name, false) {
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

				let pkgs = cache::dpkg_cache_dump(&config); // dump all the installed
				println!("Detected a dpkg database (assuming it's debian)");
				for pkg in pkgs.into_iter()  {
					let deb_pkg = DebPackage {
						control: pkg,
						kind: PkgKind::Binary,
        				signature: "NOPE".to_owned(),
						status: PackageStatus::Installed
					};
					let sqlite = config.sqlite.as_ref().unwrap();
					sqlite.add_package(deb_pkg)?;
				};

				// let pkgs = cache::cache_dump(&config); // dump all the cached packages
			
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
