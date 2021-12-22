///
/// Generic package update
/// 

use super::utils::PackageFormat;
use super::errors::InstallError;
use super::config::Config;

pub fn update(config: &mut Config) -> Result<(), InstallError> {
    match PackageFormat::from(&config.fmt) {
        PackageFormat::Deb => {
            use super::deb;
            let repos = deb::sources::DebianSource::new()?;
            deb::update(config, &repos)?;
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