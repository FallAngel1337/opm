// use itertools::Itertools;
use reqwest;
use tempfile::Builder;
use std::io;
use std::fs::{self, File};

use super::config::DebianSource;
use crate::repos::errors::InstallError;
use crate::repos::config::Config;

#[tokio::main]
pub async fn download(config: &Config, repos: &Vec<DebianSource>) -> Result<(), InstallError> {
    for (i, source) in repos.iter().enumerate() {
        println!("Source {}: {} {} {:?}", i+1, source.url, source.distribution, source.components);
        for perm in source.components.iter() {
            let release_file = format!("{}dists/{}/InRelease", source.url, source.distribution);
            let pkgcache = format!("{}dists/{}/{}/binary-amd64/Packages.xz", source.url, source.distribution, perm); // Binary packages ONLY for now
            
            let response = reqwest::get(release_file).await?;
            
            let mut dest = {
                let fname = response
                    .url()
                    .path_segments()
                    .and_then(|segments| segments.last())
                    .and_then(|name| if name.is_empty() { None } else { Some(name) })
                    .unwrap_or("tmp.bin");
        
                println!("file to download: '{}'", fname);
                let dir = config.cache.join(&source.distribution);
                fs::create_dir(&dir)?;
                let fname = dir.join(&fname);
                println!("will be located under: '{:?}'", fname);
                File::create(fname)?
            };
            let content =  response.text().await?;
            io::copy(&mut content.as_bytes(), &mut dest)?;
        }
    }
    Ok(())
}