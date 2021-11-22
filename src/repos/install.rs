///
/// Generic package install
/// 

use super::utils::Distribution;
use super::errors::InstallError;
use super::config::Config;

pub fn install(file: &str) -> Result<(), InstallError> {
    let mut config = Config::new();
    config.setup()?;

    match Distribution::get_distro() {
        Distribution::Debian => {
            use super::deb;
            deb::install(&mut config, file)?;
        }
        Distribution::Rhel => {
            println!("It's a RHEL(-based) distro");
        }
        Distribution::Other => {
            println!("Actually we do not have support for you distro!");
        }
    }

    Ok(())
}