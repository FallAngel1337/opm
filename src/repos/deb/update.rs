use xz2::read::XzDecoder;

use reqwest;

use std::io::{self, ErrorKind, prelude::*};
use std::fs::{self, File};
use std::str;

use super::sources::DebianSource;
use crate::repos::errors::InstallError;
use crate::repos::config::Config;

fn clear(config: &Config) -> Result<(), InstallError> {
    match fs::remove_dir_all(&config.cache){
        Ok(_) => (),
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => (),
            ErrorKind::NotFound => (),
            _ => panic!("fuck {}", e)
        }
    }
    match fs::remove_dir_all(&config.rls){
        Ok(_) => (),
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => (),
            ErrorKind::NotFound => (),
            _ => panic!("fuck {}", e)
        }
    }
    match fs::remove_dir_all(&config.tmp){
        Ok(_) => (),
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => (),
            ErrorKind::NotFound => (),
            _ => panic!("fuck {}", e)
        }
    }

    fs::create_dir(&config.cache)?;
    fs::create_dir(&config.rls)?;
    fs::create_dir(&config.tmp)?;

    Ok(())
}

#[tokio::main]
pub async fn update(config: &mut Config, repos: &[DebianSource]) -> Result<(), InstallError> {
    clear(config)?;

    update_releases(config, repos).await?;
    update_cache(config, repos).await?;

    Ok(())
}

async fn update_cache(config: &Config, repos: &[DebianSource]) -> Result<(), InstallError> {
    for (i, source) in repos.iter().enumerate() {
        println!("Get {}: {} {} {:?}", i+1, source.url, source.distribution, source.components);
        for perm in source.components.iter() {
            let pkgcache = format!("{}dists/{}/{}/binary-amd64/Packages.xz", source.url, source.distribution, perm); // Binary packages ONLY for now
            
            let response = reqwest::get(pkgcache).await?;
            
            let content = response.bytes().await?;
            let content: &[u8] = content.as_ref();
            
            let mut data = XzDecoder::new(content);
            let mut bytes = Vec::new();
            data.read_to_end(&mut bytes).unwrap_or_default();
            let mut bytes: &[u8] = bytes.as_ref();
            
            let url = str::replace(&source.url, "http://", "");
            let url = str::replace(&url, "/", "_");

            let pkg = config.cache.join(format!("{}{}_{}_binary-amd64_Packages", url, source.distribution, perm));
            let mut pkg = File::create(pkg)?;
            io::copy(&mut bytes, &mut pkg)?;
        };
    }

    Ok(())
}

async fn update_releases(config: &Config, repos: &[DebianSource]) -> Result<(), InstallError> {
    for (i, source) in repos.iter().enumerate() {
        println!("RLS {}: {} {} {:?}", i+1, source.url, source.distribution, source.components);
        for perm in source.components.iter() {
            let release_file = format!("{}dists/{}/InRelease", source.url, source.distribution);
            
            let response = reqwest::get(release_file).await?;
            
            let url = str::replace(&source.url, "http://", "");
            let url = str::replace(&url, "/", "_");
            
            let rls = config.rls.join(format!("{}{}_{}_binary-amd64_InRelease", url, source.distribution, perm));
            let mut dest = File::create(rls)?;

            let content =  response.text().await?;
            io::copy(&mut content.as_bytes(), &mut dest)?;
        }

    }
    Ok(())
}