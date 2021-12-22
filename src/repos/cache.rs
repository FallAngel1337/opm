// use crate::repos::deb::package::DebPackage;

use super::utils::PackageFormat;
use super::config::Config;

pub fn list_installed() {
	if let Some(pkg_fmt) = PackageFormat::get_format() {
		match pkg_fmt {
			PackageFormat::Deb => {
				use super::deb;
				deb::dump_installed()
					.into_iter()
					.for_each(|pkg| {
						println!("{} {} - {}", pkg.control.package, pkg.control.version, pkg.control.description)
					});
			},
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

pub fn search(config: &mut Config, name: &str) {
	match PackageFormat::from(&config.fmt) {
		PackageFormat::Deb => {
			use super::deb;
			deb::cache::cache_dump(config)
				.into_iter()
				.filter(|pkg| pkg.package.contains(name))
				.for_each(|pkg| {
					println!("{} {} - {}", pkg.package, pkg.version, pkg.description)
				})
		},
		PackageFormat::Rpm => {
			println!("It's a RHEL(-based) distro");
		},
		PackageFormat::Other => {
			println!("Actually we do not have support for you distro!");
		},
	}
}