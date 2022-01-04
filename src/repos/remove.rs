use anyhow::Result;
use super::{config::Config, utils::PackageFormat};

pub fn remove(config: &Config, name: &str) -> Result<()> {
    match PackageFormat::from(&config.fmt) {
        PackageFormat::Deb => {
            use super::deb;
            deb::remove(config, name)?;
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