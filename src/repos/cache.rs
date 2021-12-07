use super::utils::Distribution;
use crate::repos::config::Config;

pub fn list_installed() {
    match Distribution::get_distro() {
        Distribution::Debian => {
            // use super::deb;
            // deb::cache::cache_lookup(&mut config, file)?;
        }
        Distribution::Rhel => {
            println!("It's a RHEL(-based) distro");
        }
        Distribution::Other => {
            println!("Actually we do not have support for you distro!");
        }
    }
    
    // Ok(())
}

pub fn search(config: &Config, name: &str) {
    match Distribution::get_distro() {
        Distribution::Debian => {
            use super::deb;
            deb::cache::cache_lookup(config, name);
        }
        Distribution::Rhel => {
            println!("It's a RHEL(-based) distro");
        }
        Distribution::Other => {
            println!("Actually we do not have support for you distro!");
        }
    }
    
    // Ok(())
}