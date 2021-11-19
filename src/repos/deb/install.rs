///
/// Debian package install
/// 

use crate::repos::errors::InstallError;
use crate::repos::config;
use super::extract::extract;
use crate::repos::database::{Packages, SQLite};

pub fn install(file: &str) -> Result<(), InstallError> {
    let mut sqlite = SQLite::new("installed.db");
    sqlite.init()?;
    sqlite.create_tables()?;

    println!("Downloading ...");
    let pkg = extract(file, config::TMPDIR)?;
    sqlite.register(Packages::DebPackage(pkg))?;
    // println!("{:?}", pkg.control);
    println!("Extracting ...");
    println!("Installing ...");


    Ok(())
}