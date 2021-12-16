use std::path::PathBuf;

use crate::repos::errors::InstallError;
use crate::repos::config::Config;

use super::cache;

// use super::dependencies;

#[tokio::main]
pub async fn download(config: &Config, name: &str) -> Result<PathBuf, InstallError> {
    println!("Downloading {} from {:?}", name, config.cache);

    if let Some(pkg) = cache::cache_lookup(config, name) {
        println!("FOUND => {:?}", pkg);
    } else {
        println!("Not found");
    }

    Ok(PathBuf::new())
}