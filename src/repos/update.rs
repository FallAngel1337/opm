///
/// Generic package update
/// 

use super::utils::Distribution;
use super::errors::InstallError;
use super::config::Config;

pub fn update() -> Result<(), InstallError> {
    let mut config = Config::new();
    config.setup()?;
    println!("Current config: {:?}", config);

    match Distribution::get_distro() {
        Distribution::Debian => {
            use super::deb;
            let repos = deb::sources::DebianSource::new()?;
            println!("It's a Debian(-based) distro");
            deb::update(&mut config, &repos)?;
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