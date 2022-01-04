use anyhow::Result;
use super::cache;
use crate::repos::{errors::InstallError, config::Config};

pub fn remove(config: &Config, name: &str) -> Result<()> {
    if let Some(pkg) = cache::check_installed(config, name) {
        println!("Removing {} ...", pkg.control.package);
        Ok(())
    } else {
        anyhow::bail!(InstallError::NotFoundError(name.to_string()));
    }
}