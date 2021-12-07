use super::utils::Distribution;
use crate::repos::config::Config;

pub fn list_installed() {
    match Distribution::get_distro() {
        Distribution::Debian => {
            println!("Still working on this");
        }
        Distribution::Rhel => {
            println!("It's a RHEL(-based) distro");
        }
        Distribution::Other => {
            println!("Actually we do not have support for you distro!");
        }
    }
}

pub fn search(config: &Config, name: &str) {
    match Distribution::get_distro() {
        Distribution::Debian => {
            use super::deb;
            let pkgs = deb::cache::cache_lookup(config, name);
            pkgs.iter().for_each(|pkg| {
                println!("{} {} - {} ({})", pkg.0.package, pkg.0.version, pkg.0.description, pkg.1);
            })
        }
        Distribution::Rhel => {
            println!("It's a RHEL(-based) distro");
        }
        Distribution::Other => {
            println!("Actually we do not have support for you distro!");
        }
    }
}