///
/// Generic package update
/// 

use super::utils::PackageFormat;
use super::errors::InstallError;
use super::config::Config;

pub fn update() -> Result<(), InstallError> {
    let mut config = Config::new()?;
    println!("Current config: {:?}", config);

    if let Some(pkg_fmt) = PackageFormat::get_format() {
        match pkg_fmt {
            PackageFormat::Deb => {
                use super::deb;
                let repos = deb::sources::DebianSource::new()?;
                deb::update(&mut config, &repos)?;
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