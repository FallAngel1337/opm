///
/// Generic package install
/// 

use super::database::SQLite;
use super::utils::PackageFormat;
use super::errors::InstallError;
use super::config::Config;

pub fn install(config: &mut Config, file: &str) -> Result<(), InstallError> {
    let mut sqlite = SQLite::new(&mut config.pkgs);
    match sqlite.init() {
        Ok(_) => (),
        Err(err) => return Err(InstallError::DataBaseError(err, "Could not start the database".to_owned()))
    }

    if let Some(pkg_fmt) = PackageFormat::get_format() {
        match pkg_fmt {
            PackageFormat::Deb => {
                use super::deb;
                deb::install(config, file)?;
            }
            PackageFormat::Rpm => {
                println!("It's a RHEL(-based) distro");
            }
            PackageFormat::Other => {
                println!("Actually we do not have support for you distro!");
            }
        }
    }

    Ok(())
}