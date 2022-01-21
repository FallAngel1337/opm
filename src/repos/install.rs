///
/// Generic package install
/// 

use anyhow::Result;
use super::utils::PackageFormat;
use super::config::Config;

#[tokio::main]
pub async fn install(config: &mut Config, name: &str, force: bool) -> Result<()> {
    match PackageFormat::from(&config.pkg_fmt) {
        PackageFormat::Deb => {
            use super::deb;
            deb::install(config, name, force).await?; 
        }
        PackageFormat::Rpm => {
            println!("It's a RHEL(-based) distro");
        }
        PackageFormat::Other => {
            println!("Actually we do not have support for you distro!");
        }
    }

    Ok(())
}