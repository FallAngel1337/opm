use std::{path::PathBuf, fs::File, io};

use crate::repos::errors::InstallError;
use crate::repos::config::Config;

use super::package::DebPackage;

use reqwest;

#[tokio::main]
pub async fn download(config: &mut Config, pkg: &DebPackage) -> Result<PathBuf, InstallError> {
    println!("Downloading {} from {:?} ...", pkg.control.package, config.cache);

    let response = reqwest::get(format!("http://{}", pkg.control.filename)).await?;
    
    let content = response.bytes().await?;
    let mut content: &[u8] = content.as_ref();

    let name = pkg.control.filename.split("/").last().unwrap();
    println!("Downloading {} ...", name);
    let name = format!("{}/{}", config.tmp.clone().into_os_string().into_string().unwrap(), name);
    println!("Saving to {}", name);

    let mut pkg = File::create(&name)?;
    io::copy(&mut content, &mut pkg)?;

    Ok(PathBuf::from(name))
}