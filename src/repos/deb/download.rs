use anyhow::Result;
use bytesize::ByteSize;
use std::{path::PathBuf, fs::File, io};

use crate::repos::config::Config;
use super::{package::DebPackage, signatures};
use reqwest;

pub async fn download(config: &Config, pkg: DebPackage) -> Result<(PathBuf, String)> {
    let control = pkg.control.clone();
    let url = format!("http://{}", control.filename);    
    let response = reqwest::get(&url).await?;

    let content = response.bytes().await?;
    let mut content: &[u8] = content.as_ref();

    println!("Get: {} {} {} {} [{}]", url, control.architecture,
    control.package, control.version, ByteSize(content.len() as u64));

    match signatures::verify_sig(&pkg, content) {
        Ok(()) => (),
        Err(e) => eprintln!("Failed to check package signature :: {}", e)
    }

    let name = control.filename.split('/').last().unwrap().to_string();
    let fname = format!("{}/{}", config.tmp, name);

    let mut pkg = File::create(&fname)?;
    io::copy(&mut content, &mut pkg)?;

    Ok((PathBuf::from(&fname), control.package))
}