///
/// Debian package install
/// 

use crate::repos::errors::InstallError;
use crate::repos::database::{Packages, SQLite};
use crate::repos::config::Config;
use super::extract;
use super::config::DebianSource;
use super::download;

pub fn install(config: &Config, file: &str) -> Result<(), InstallError> {
    let mut sqlite = SQLite::new(&config.tmp);
    sqlite.init()?;
    sqlite.create_tables()?;

    let list = DebianSource::new()?;
    // println!("LIST => {:?}", list);
    download::download(config, &list)?;
    
    // let pkg = extract(file, config::TMPDIR)?;
    // println!("Installing {}", pkg.control.package);
    // println!("Downloading ...");
    // sqlite.register(Packages::DebPackage(pkg))?;
    // println!("Extracting ...");
    
    Ok(())
}