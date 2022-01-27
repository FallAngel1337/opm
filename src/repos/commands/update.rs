///
/// Generic package update
/// 

use anyhow::Result;
use super::packages::PackageFormat;
use super::config::Config;

#[tokio::main]
pub async fn update(config: &mut Config) -> Result<()> {
    match config.os_info.default_package_format {
        PackageFormat::Deb => {
            use super::deb;
            let repos = deb::sources::DebianSource::new()?;
            deb::update(config, &repos).await?;
        }
        PackageFormat::Rpm => {
            println!("It's a RHEL(-based) distro");
        }
        PackageFormat::Pkg => {
            println!("It's a RHEL(-based) distro");
        }
        PackageFormat::Unknown => {
            println!("Actually we do not have support for you distro!");
        }
    }

    Ok(())
}

pub fn clear(config: &Config) -> Result<()> {
    match config.os_info.default_package_format {
        PackageFormat::Deb => {
            use super::deb;
            deb::clear(config)?;
        }
        PackageFormat::Rpm => {
            println!("It's a RHEL(-based) distro");
        }
        PackageFormat::Pkg => {
            println!("It's a RHEL(-based) distro");
        }
        PackageFormat::Unknown => {
            println!("Actually we do not have support for you distro!");
        }
    }

    Ok(())
}