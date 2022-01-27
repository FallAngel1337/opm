use anyhow::Result;
use super::packages::PackageFormat;
use super::config::Config;
 
pub fn list_installed(config: &Config) {
	match config.os_info.default_package_format {
		PackageFormat::Deb => {
			use super::deb;
			let dump = deb::db_dump(config);
			dump
			.iter()
			.for_each(|pkg| {
				println!("{} {} - {}", pkg.control.package, pkg.control.version, pkg.control.description)
			});

			println!("Found {} packages installed", dump.len())
		},
		PackageFormat::Rpm => {
			println!("It's a RHEL(-based) distro");
		}
		PackageFormat::Pkg => {
			println!("It's a Arch(-based) distro");
		},
		PackageFormat::Unknown => {
			println!("Actually we do not have support for you distro!");
		}
	}
}


pub fn search(config: &mut Config, name: &str) -> Result<()> {
	match config.os_info.default_package_format {
		PackageFormat::Deb => {
			use super::deb;
			let result = deb::cache::cache_search(config, name)?;
			if let Some(pkgs) = result {
				pkgs
				.iter()
				.for_each(|pkg| {
					println!("{} - {}", pkg.control.package, pkg.control.description);
				});
				
				println!("Found {} packages for `{}`", pkgs.len(), name);
			}

			Ok(())
		},
		PackageFormat::Rpm => {
			println!("It's a RHEL(-based) distro");
			Ok(())
		},
		PackageFormat::Pkg => {
			println!("It's a Arch(-based) distro");
			Ok(())
		},
		PackageFormat::Unknown => {
			println!("Actually we do not have support for you distro!");
			Ok(())
		},
	}
}