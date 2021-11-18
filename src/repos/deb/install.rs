///
/// Debian package install
/// 

use crate::repos::errors::InstallError;
use crate::repos::config;
use super::extract::extract;

pub fn install(file: &str) -> Result<(), InstallError> {
    println!("Downloading ...");
    let pkg = extract(file, config::TMPDIR)?;
    println!("Extracting ...");
    println!("Installing ...");

    println!("{:?}", pkg.control);

    Ok(())
}