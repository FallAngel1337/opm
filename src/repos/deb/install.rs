///
/// Debian package install
/// 

use crate::repos::errors::InstallError;
use crate::repos::config;
use super::extract::extract;
use crate::repos::database::{Packages, SQLite};

pub fn install(file: &str) -> Result<(), InstallError> {
    let mut sqlite = SQLite::new(config::INSTALL_DBPATH);
    sqlite.init()?;
    sqlite.create_tables()?;
    
    let pkg = extract(file, config::TMPDIR)?;
    println!("Installing {}", pkg.control.package);
    println!("Downloading ...");
    sqlite.register(Packages::DebPackage(pkg))?;
    println!("Extracting ...");

    Ok(())
}