///
/// Debian package install
/// 

use crate::repos::errors::InstallError;
use crate::repos::config::Config;
use super::cache;
use super::{extract, download};

// TODO: Check for newer versions of the package if installed
pub fn install(config: &mut Config, name: &str) -> Result<(), InstallError> {
    if name.ends_with(".deb") {
        let _pkg = extract::extract(config, name, &config.tmp.clone())?;
        //TODO: Verify if this package is alredy installed
        println!("Extracting ...");
    } else {
        let pkg = cache::db_lookup(config, name, true, false)?;
        if !pkg.is_empty() {
            let pkg = pkg.into_iter().next().unwrap();
            println!("{} is already installed\nFound:", name);
            println!("{} - {}", pkg.control.package, pkg.control.version);
            return Err(InstallError::AlreadyInstalled);
        }

        // Downloand and call install on the downloaded packages
        println!("Downloading {} for debian ...", name);
        download::download(config, name)?;
    }
    

    Ok(())
}