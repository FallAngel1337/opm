///
/// Debian package install
/// 

use crate::repos::errors::InstallError;
use crate::repos::database::{Packages, SQLite};
use crate::repos::config;
use super::extract::extract;
use super::config::ReposList;
use super::download;

pub fn install(file: &str) -> Result<(), InstallError> {
    let mut sqlite = SQLite::new(config::INSTALL_DBPATH);
    sqlite.init()?;
    sqlite.create_tables()?;

    let list = ReposList::new()?;

    
    let pkg = extract(file, config::TMPDIR)?;
    println!("Installing {}", pkg.control.package);
    println!("Downloading ...");
    sqlite.register(Packages::DebPackage(pkg))?;
    println!("Extracting ...");

    Ok(())
}