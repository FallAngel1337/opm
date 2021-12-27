use anyhow::Result;
use std::{path::PathBuf, fs::File, io};

use crate::repos::config::Config;

use super::package::DebPackage;

use reqwest;

pub async fn download(config: &Config, pkg: &DebPackage) -> Result<PathBuf> {
    println!("Downloading {} ...", pkg.control.package);

    let response = reqwest::get(format!("http://{}", pkg.control.filename)).await?;
    
    let content = response.bytes().await?;
    let mut content: &[u8] = content.as_ref();

    let name = pkg.control.filename.split('/').last().unwrap();
    let name = format!("{}/{}", config.tmp, name);

    let mut pkg = File::create(&name)?;
    io::copy(&mut content, &mut pkg)?;

    Ok(PathBuf::from(name))
}