// use super::config::DebianSource;
use crate::repos::errors::InstallError;
use crate::repos::config::Config;

use super::dependencies;

#[tokio::main]
pub async fn download(config: &Config, name: &str) -> Result<(), InstallError> {
    println!("Downloading {} from {:?}", name, config.cache);

    // if let Some(dep) = dependencies::
    //     // dependencies::solve_dependencies(config, );

    Ok(())
}