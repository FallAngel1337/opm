use anyhow::Result;
use super::{config::Config, packages::PackageFormat};

pub fn remove(config: &Config, name: &str, purge: bool) -> Result<()> {
    match config.os_info.default_package_format {
        PackageFormat::Deb => {
            use super::deb;
            deb::remove(config, name, purge)?;
        }
        PackageFormat::Rpm => {
            println!("It's a RHEL(-based) distro");
        }
        PackageFormat::Pkg => {
            println!("It's a Arch(-based) distro");
        }
        PackageFormat::Unknown => {
            println!("Actually we do not have support for you distro!");
        }
    }

    Ok(())
}