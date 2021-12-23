///
/// Generic package install
/// 

use anyhow::Result;
use super::utils::PackageFormat;
use super::config::Config;

pub fn install(config: &mut Config, name: &str) -> Result<()> {
    match PackageFormat::from(&config.fmt) {
        PackageFormat::Deb => {
            use super::deb;
            deb::install(config, name)?; 
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