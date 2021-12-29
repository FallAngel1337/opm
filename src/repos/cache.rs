use anyhow::Result;
use super::utils::PackageFormat;
use super::config::Config;
 
pub fn list_installed(config: &Config) {
	match PackageFormat::from(&config.fmt) {
		PackageFormat::Deb => {
			use super::deb;
			deb::db_dump(config)
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
}


pub fn search(config: &mut Config, name: &str) -> Result<()> {
	match PackageFormat::from(&config.fmt) {
		PackageFormat::Deb => {
			use super::deb;
			deb::cache::cache_dump(config)?
				.into_iter()
				.filter(|pkg| pkg.package.contains(name))
				.for_each(|pkg| {
					println!("{} {} - {}", pkg.package, pkg.version, pkg.description)
				});
			Ok(())
		},
		PackageFormat::Rpm => {
			println!("It's a RHEL(-based) distro");
			Ok(())
		},
		PackageFormat::Other => {
			println!("Actually we do not have support for you distro!");
			Ok(())
		},
	}
}