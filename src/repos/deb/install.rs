///
/// Debian package install
/// 

use crate::repos::errors::InstallError;
use crate::repos::config::Config;
use crate::repos::cache as opm_cache;
use super::extract;
use super::download;

// TODO: Check for newer versions of the package if installed
pub fn install(config: &mut Config, name: &str) -> Result<(), InstallError> {
    if name.ends_with(".deb") {
        let _pkg = extract::extract(config, name, &config.tmp)?;
        println!("Extracting ...");
    } else {
        if let Some(pkg) = opm_cache::cache_lookup(&config, name, true) {
            let pkg = pkg.into_iter().next().unwrap();
            println!("{} is already installed\nFound:", name);
            println!("{} - {}", pkg.package, pkg.version);
        } else {
            println!("{} can be installed", name);
            download::download(config, name)?;
            println!("Installing {}", name);
        }
    }
    

    Ok(())
}