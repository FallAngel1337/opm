use anyhow::Result;
use std::{path::PathBuf, fs::File, io};

use crate::repos::config::Config;
use super::package::DebPackage;
use reqwest;

pub async fn download<'a>(config: &Config, pkg: DebPackage) -> Result<(PathBuf, String)> {
    let pkg_name = pkg.control.package;
    println!("Downloading {} ...", pkg_name);

    let response = reqwest::get(format!("http://{}", pkg.control.filename)).await?;
    
    let content = response.bytes().await?;
    let mut content: &[u8] = content.as_ref();

    let name = pkg.control.filename.split('/').last().unwrap().to_string();
    let fname = format!("{}/{}", config.tmp, name);

    let mut pkg = File::create(&fname)?;
    io::copy(&mut content, &mut pkg)?;

    Ok((PathBuf::from(&fname), pkg_name))
}