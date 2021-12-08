use super::{utils::Distribution, deb::package::ControlFile};
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

	let control = if exact_match {
		fs::read_to_string(path)
		.unwrap()
		.split("\n\n")
		.map(|ctrl| ControlFile::from(ctrl).unwrap())
		.filter(|ctrl| ctrl.package == name)
		.collect::<Vec<_>>()
	} else {
		fs::read_to_string(path)
		.unwrap()
		.split("\n\n")
		.map(|ctrl| ControlFile::from(ctrl).unwrap())
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
    match Distribution::get_distro() {
        Distribution::Debian => {
            println!("Still working on this");
        }
        Distribution::Rhel => {
            println!("It's a RHEL(-based) distro");
        }
        Distribution::Other => {
            println!("Actually we do not have support for you distro!");
        }
    }
}

pub fn search(config: &Config, name: &str) {
    match Distribution::get_distro() {
        Distribution::Debian => {
            if let Some(pkgs) = cache_lookup(config, name, false) {
                pkgs.iter().for_each(|pkg| {
                    println!("{} {} - {} ({})", pkg.0.package, pkg.0.version, pkg.0.description, pkg.1);
                })
            }
        }
        Distribution::Rhel => {
            println!("It's a RHEL(-based) distro");
        }
        Distribution::Other => {
            println!("Actually we do not have support for you distro!");
        }
    }
}