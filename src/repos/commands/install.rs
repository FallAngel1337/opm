///
/// Generic package install
/// 

use anyhow::Result;
use super::packages::PackageFormat;
use super::config::Config;

#[tokio::main]
pub async fn install(config: &mut Config, name: &str, force: bool) -> Result<()> {
    match config.os_info.default_package_format {
        PackageFormat::Deb => {
            use super::deb;
            deb::install(config, name, force).await?; 
        }
        PackageFormat::Rpm => {
            println!("It's a RHEL(-based) distro");
        }
        PackageFormat::Pkg => {
            println!("It's a Arch(-based) distro");
        }
        PackageFormat::Unknown => {
            println!("Actually we do not have support for you distro!");
        }
    }

    Ok(())
}