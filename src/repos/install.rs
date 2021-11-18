///
/// Generic package install
/// 

use super::utils::Distribution;
use super::errors::InstallError;

pub fn install(file: &str) -> Result<(), InstallError> {
    match Distribution::get_distro() {
        Distribution::Debian => {
            use super::deb;
            println!("It's a Debian(-based) distro");
            deb::install(file)?;
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