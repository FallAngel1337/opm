use anyhow::Result;
use super::{config::Config, os_fingerprint::PackageFormat};

pub fn remove(config: &Config, name: &str, purge: bool) -> Result<()> {
    match PackageFormat::from(&config.pkg_fmt) {
        PackageFormat::Deb => {
            use super::deb;
            deb::remove(config, name, purge)?;
        }
        PackageFormat::Rpm => {
            println!("It's a RHEL(-based) distro");
        }
        PackageFormat::Other => {
            println!("Actually we do not have support for you distro!");
        }
    }

    Ok(())
}