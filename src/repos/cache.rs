use super::{utils::PackageFormat, deb::package::ControlFile};
use crate::repos::config::Config;
use std::fs;

///
/// Lookup into the local cache(~/.rpm/cache)
/// 
// TODO: Improve it to be less slow
pub fn cache_lookup(config: &Config, name: &str, exact_match: bool) -> Option<Vec<(ControlFile, String)>> {
	let mut pkgs = Vec::new();

	for entry in fs::read_dir(&config.cache).unwrap() {
		let entry = entry.unwrap();
		let path = entry.path();
		
		let control = fs::read_to_string(path).unwrap();

		let control = control
			.split("\n\n")
			.map(|crtl| ControlFile::from(crtl).unwrap());

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

		control.into_iter().for_each(|pkg| {
			let url = format!("{}/ubuntu/{}", url, &pkg.filename);
			pkgs.push((pkg, String::from(&url)));
		});
	}

	if pkgs.len() > 0 {
		Some(pkgs)
	} else {
		None
	}
}

pub fn list_installed() {
    match PackageFormat::get_format() {
        PackageFormat::Deb => {
            println!("Still working on this");
        }
        PackageFormat::Rpm => {
            println!("It's a RHEL(-based) distro");
        }
        PackageFormat::Other => {
            println!("Actually we do not have support for you distro!");
        }
    }
}

pub fn search(config: &Config, name: &str) {
    if let Some(pkg_fmt) = PackageFormat::get_distro() {
		match pkg_fmt {
			PackageFormat::Deb => {
				if let Some(pkgs) = cache_lookup(config, name, false) {
					pkgs.iter().for_each(|pkg| {
						println!("{} {} - {} ({})", pkg.0.package, pkg.0.version, pkg.0.description, pkg.1);
					})
				}
			}
			PackageFormat::Rpm => {
				println!("It's a RHEL(-based) distro");
			}
			PackageFormat::Other => {
				println!("Actually we do not have support for you distro!");
			}
		}
    }
}