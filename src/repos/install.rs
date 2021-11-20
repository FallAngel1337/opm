///
/// Generic package install
/// 

use super::utils::Distribution;
use super::errors::InstallError;
use super::config::Config;

pub fn install(file: &str) -> Result<(), InstallError> {
    let mut config = Config::new();
    println!("Current config: {:?}", config);

    match Distribution::get_distro() {
        Distribution::Debian => {
            use super::deb;
            println!("It's a Debian(-based) distro");
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