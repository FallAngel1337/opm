use crate::repos::{deb::package::DebPackage, database::PackageStatus};

use super::{utils::PackageFormat, database::{GenericPackage, Package}};
use super::config::Config;
use rusqlite::Result;

pub fn list_installed(config: &mut Config) {
	config.setup_db().expect("Failed to open the database");
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

pub fn lookup(config: &mut Config, name: &str) -> Option<Vec<GenericPackage>> {
	config.setup_db().expect("Failed to open the database");

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
					// FIXME: Exact match query
					if let Ok(pkg) = sqlite.lookup(name, false) {
						if let Some(pkg) = pkg {
							println!("{:?}", pkg);
						} else {}
						println!("A");
					} else {
						println!("B");
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

///
/// Find all installed packages and then insert 'em into the database
/// 
//TODO: Make better to read
pub fn populate_db(config: &Config) -> Result<()> {
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
        				signature: "NOPE".to_owned(), // TODO: Get the real signature
						status: PackageStatus::Installed
					};

					if let Some(sqlite) = config.sqlite.as_ref() {
						sqlite.add_package(deb_pkg, false)?;
					}
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

pub fn update_cache(config: &Config) -> Result<()> {
	if let Some(pkg_fmt) = PackageFormat::get_format() {
		match pkg_fmt {
			PackageFormat::Deb => {
				use super::deb::{cache, package::PkgKind};

				let pkgs = cache::cache_dump(&config); // dump all the installed
				println!("Updating the database (assuming it's debian)");
				for pkg in pkgs.into_iter()  {
					let deb_pkg = DebPackage {
						control: pkg,
						kind: PkgKind::Binary,
        				signature: "NOPE".to_owned(), // TODO: Get the real signature
						status: PackageStatus::Installed
					};
					let sqlite = config.sqlite.as_ref().unwrap();
					sqlite.add_package(deb_pkg, true)?;
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

pub fn load_into<P: Package>(config: &mut Config, data: Vec<P>) -> Result<()> {
	if let Some(sqlite) = config.sqlite.as_ref() {
		for pkg in data.into_iter() {
			sqlite.add_package(pkg, true)?;
		}
	}

	Ok(())
}