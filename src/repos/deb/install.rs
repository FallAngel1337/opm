///
/// Debian package install
/// 

use crate::repos::errors::InstallError;
use crate::repos::config::Config;
use super::extract;
use super::download;
use super::cache;

pub fn install(config: &mut Config, name: &str) -> Result<(), InstallError> {
    if name.ends_with(".deb") {
        let _pkg = extract::extract(name, &config.tmp)?;
        println!("Extracting ...");
    } else {
        cache::cache_lookup(&config, name);
        download::download(config, name)?;
    }
    
    println!("Installing {}", name);

    Ok(())
}