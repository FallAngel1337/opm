use std::{path::PathBuf, fs::File, io};

use crate::repos::errors::InstallError;
use crate::repos::config::Config;

use super::cache;

use reqwest;

#[tokio::main]
pub async fn download(config: &mut Config, name: &str) -> Result<PathBuf, InstallError> {
    println!("Downloading {} from {:?} ...", name, config.cache);

    if let Some(pkg) = cache::cache_lookup(config, name) {
        println!("FOUND => {:?}", pkg.control.package);
        println!("DEPENDS ON => {:#?}", pkg.control.depends);
        let response = reqwest::get(format!("http://{}", pkg.control.filename)).await?;
        
        let content = response.bytes().await?;
        let mut content: &[u8] = content.as_ref();

        let name = pkg.control.filename.split("/").last().unwrap();
        println!("Downloading {} ...", name);
        let name = format!("{}/{}", config.tmp.clone().into_os_string().into_string().unwrap(), name);
        println!("Saving to {}", name);

        let mut pkg = File::create(name)?;
        io::copy(&mut content, &mut pkg)?;

    } else {
        println!("Package {} not found! Try update then try again.", name);
    }


    Ok(PathBuf::new())
}