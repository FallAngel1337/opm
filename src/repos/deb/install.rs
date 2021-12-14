///
/// Debian package install
/// 

use crate::repos::errors::InstallError;
use crate::repos::config::Config;
use crate::repos::cache as opm_cache;
use super::{extract, download};

// TODO: Check for newer versions of the package if installed
pub fn install(config: &mut Config, name: &str) -> Result<(), InstallError> {
    if name.ends_with(".deb") {
        let _pkg = extract::extract(config, name, &config.tmp)?;
        println!("Extracting ...");
    } else {
        if let Some(pkg) = opm_cache::lookup(&config, name) {
            if let Some(pkg) = pkg.into_iter().next() {
                println!("{} is already installed\nFound:", name);
                println!("{} - {}", pkg.name, pkg.version);
            }
        } else {
            println!("{} can be installed", name);
        }
    }
    

    Ok(())
}