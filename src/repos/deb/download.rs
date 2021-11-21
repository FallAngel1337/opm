use super::config::DebianSource;
use crate::repos::errors::InstallError;
use crate::repos::config::Config;

use super::update;

#[tokio::main]
pub async fn download(config: &Config, repos: &Vec<DebianSource>) -> Result<(), InstallError> {
    update::update(config, repos).await?;

    Ok(())
}