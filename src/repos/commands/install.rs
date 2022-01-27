
///
/// Generic package install
/// 

use anyhow::Result;
use indicatif::HumanDuration;
use super::packages::PackageFormat;
use std::time::Instant;
use super::config::Config;

#[tokio::main]
pub async fn install(config: &mut Config, name: &str, force: bool) -> Result<()> {
    let start = Instant::now();

    match config.os_info.default_package_format {
        PackageFormat::Deb => {
            use super::deb;
            deb::install(config, name, force).await?; 
        },
        PackageFormat::Rpm => {
            println!("It's a RHEL(-based) distro");
        },
        PackageFormat::Pkg => {
            println!("It's a Arch(-based) distro");
        },
        PackageFormat::Unknown => {
            println!("Actually we do not have support for you distro!");
        },
    }
    let duration = start.elapsed();
    println!("Installed {} in {}", name, HumanDuration(duration));

    Ok(())
}