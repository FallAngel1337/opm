///
/// Debian package install
/// 

use crate::repos::errors::InstallError;
use crate::repos::database::SQLite;
use crate::repos::config::Config;
use super::extract;
use super::config::DebianSource;
use super::download;

pub fn install(config: &mut Config, file: &str) -> Result<(), InstallError> {
    
    let list = DebianSource::new()?;
    download::download(config, &list)?;

    let mut sqlite = SQLite::new(&mut config.pkgs);
    println!("Current config: {:?}", sqlite.db);
    sqlite.init()?;
    sqlite.create_tables()?;

    
    let pkg = extract::extract(file, &config.tmp)?;
    println!("Installing {}", pkg.control.package);
    sqlite.register(pkg)?;
    println!("Extracting ...");
    
    Ok(())
}