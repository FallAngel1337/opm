///
/// Debian package install
/// 

use crate::repos::{errors::InstallError, deb::dependencies};
use crate::repos::config::Config;
use super::cache;
use super::{extract, download};

// TODO: Check for newer versions of the package if installed
pub fn install(config: &mut Config, name: &str) -> Result<(), InstallError> {
    if name.ends_with(".deb") {
        let _pkg = extract::extract(name, &config.tmp)?;
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

        if let Some(pkg) = cache::cache_lookup(config, name) {
            println!("FOUND => {:?}", pkg.control.package);
            dependencies::get_dependencies(config, &pkg);
            // if let Some(dep) = pkg.control.depends {
            //     println!("DEPENDS ON => {:#?}", dep);
            //     dep.into_iter().for_each(|name| {
            //     if dependencies::check_dependencie(config, &name).is_none() {
            //         println!("Need to install {:?}", name);
            //         // download(config, &name).await;
            //     }
            //     });
            //     // download::download(config, name)?;
            // }
        }

    }
    

    Ok(())
}