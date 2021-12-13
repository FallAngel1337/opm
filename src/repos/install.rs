///
/// Generic package install
/// 

use super::utils::PackageFormat;
use super::errors::InstallError;
use super::config::Config;

pub fn install(config: &mut Config, file: &str) -> Result<(), InstallError> {
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
    } else {
        eprintln!("Consider define `PKG_FMT` environment variable!");
        std::process::exit(1);
    }

    Ok(())
}