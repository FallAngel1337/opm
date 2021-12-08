///
/// Debian package install
/// 

use crate::repos::errors::InstallError;
use crate::repos::config::Config;
// use crate::repos::cache as rpm_cache;
use super::extract;
use super::download;
use super::cache;
// use deb_version;

// TODO: Check for newer versions of the package if installed
pub fn install(config: &mut Config, name: &str) -> Result<(), InstallError> {
    if name.ends_with(".deb") {
        let _pkg = extract::extract(name, &config.tmp)?;
        println!("Extracting ...");
    } else {
        if let Some(pkg) = cache::dpkg_cache_lookup(name, true) {
            // let cached_pkgs = rpm_cache::cache_lookup(&config, name);
            println!("{} is already installed\nFound:", name);
            println!("{} - {}", pkg.package, pkg.version);
            // println!("-> {:?}", deb_version::compare_versions(&pkg.version, &pkg.version));
        } else {
            println!("{} need to be installed", name);
            download::download(config, name)?;
            println!("Installing {}", name);
        }
    }
    

    Ok(())
}