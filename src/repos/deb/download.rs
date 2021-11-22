// use super::config::DebianSource;
use crate::repos::errors::InstallError;
use crate::repos::config::Config;

#[tokio::main]
pub async fn download(config: &Config, name: &str) -> Result<(), InstallError> {
    println!("Downloading {} from {:?}", name, config.cache);

    Ok(())
}