///
/// Debian package install
/// 

use crate::repos::errors::InstallError;
use crate::repos::database::SQLite;
use crate::repos::config::Config;
use super::extract;
use super::download;

pub fn install(config: &mut Config, name: &str) -> Result<(), InstallError> {
    let mut sqlite = SQLite::new(&mut config.pkgs);
    sqlite.init()?;
    sqlite.create_tables()?;
    
    if name.ends_with(".deb") {
        let _pkg = extract::extract(name, &config.tmp)?;
        // sqlite.register(pkg)?;
        println!("Extracting ...");

    } else {
        download::download(config, name)?;

    }
    println!("Installing {}", name);

        
    
    Ok(())
}