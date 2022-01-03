use anyhow::Result;
use xz2::read::XzDecoder;
use flate2::read::GzDecoder;
use std::{
    io::{ErrorKind, prelude::*},
    fs,
    path::Path,
    str
};
use reqwest;
use futures::future;
use super::sources::DebianSource;
use crate::repos::config::Config;

fn unpack(filename: &str, data: &[u8], bytes: &mut Vec<u8>) {
    if filename.ends_with(".gz") {
        let mut tar = GzDecoder::new(data);
        tar.read_to_end(bytes).unwrap_or_default();
    } else if filename.ends_with(".xz") {
        let mut tar = XzDecoder::new(data);
        tar.read_to_end(bytes).unwrap_or_default();
    }
}

pub fn clear(config: &Config) -> Result<()> {
    match fs::remove_dir_all(&config.cache){
        Ok(_) => (),
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => (),
            ErrorKind::NotFound => (),
            _ => panic!("Got and unexpected error :: {}", e)
        }
    }
    match fs::remove_dir_all(&config.rls){
        Ok(_) => (),
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => (),
            ErrorKind::NotFound => (),
            _ => panic!("Got and unexpected error :: {}", e)
        }
    }
    match fs::remove_dir_all(&config.tmp){
        Ok(_) => (),
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => (),
            ErrorKind::NotFound => (),
            _ => panic!("Got and unexpected error :: {}", e)
        }
    }

    fs::create_dir(&config.cache)?;
    fs::create_dir(&config.rls)?;
    fs::create_dir(&config.tmp)?;

    Ok(())
}

pub async fn update(config: &mut Config, repos: &[DebianSource]) -> Result<()> {
    update_releases(config, repos).await?;
    update_cache(config, repos).await?;

    Ok(())
}

async fn update_cache(config: &Config, repos: &[DebianSource]) -> Result<()> {
    let mut tasks = vec![];
    for (i, source) in repos.iter().enumerate() {
        for perm in source.components.iter() {
            // Binary packages ONLY for now
            let pkgcache = format!("{}dists/{}/{}/binary-amd64/Packages.xz", source.url, source.distribution, perm);
            let response = match reqwest::get(&pkgcache).await {
                Ok(r) => Some(r),
                Err(_) => {
                    let pkgcache = format!("{}dists/{}/{}/binary-amd64/Packages.gz", source.url, source.distribution, perm);
                    match reqwest::get(&pkgcache).await {
                        Ok(r) => Some(r),
                        Err(e) => {
                            eprintln!("Could not get the package at {} due {}", pkgcache, e);
                            None
                        }
                    }
                }
            };

            let url = str::replace(&source.url, "http://", "");
            let url = str::replace(&url, "/", "_");

            let pkg = Path::new(&config.cache).join(format!("{}dists_{}_{}_binary-amd64_Packages", url, source.distribution, perm));
            
            tasks.push(tokio::spawn(async move {
                if let Some(res) = response {
                    let content = res.bytes().await.unwrap();
                    println!("Hit {}: {} [{} kB]", i+1, pkgcache, content.len() / 1024);
                    
                    let mut bytes = Vec::new();
                    unpack(&pkgcache, content.as_ref(), &mut bytes);
                    let mut bytes: &[u8] = bytes.as_ref();
                    if !bytes.is_empty() {
                        let mut pkg = tokio::fs::File::create(pkg).await.unwrap();
                        tokio::io::copy(&mut bytes, &mut pkg).await.unwrap();
                    }
                }

            }));
        };
    }

    future::join_all(tasks).await;
    Ok(())
}

async fn update_releases(config: &Config, repos: &[DebianSource]) -> Result<()> {
    let mut tasks = vec![];
    
    for (i, source) in repos.iter().enumerate() {
        for perm in source.components.iter() {
            let release_file = format!("{}dists/{}/InRelease", source.url, source.distribution);
            
            let url = str::replace(&source.url, "http://", "");
            let url = str::replace(&url, "/", "_");
            
            let rls = Path::new(&config.rls).join(format!("{}dists_{}_{}_binary-amd64_InRelease", url, source.distribution, perm));
            
            tasks.push(tokio::spawn(async move {
                let response = reqwest::get(&release_file).await.unwrap();
                let content =  response.text().await.unwrap();
                println!("Hit {}: {} [{} kB]", i+1, release_file, content.len() / 1024);
                
                let mut dest = tokio::fs::File::create(rls).await.unwrap();
                tokio::io::copy(&mut content.as_bytes(), &mut dest).await.unwrap();
            }));
        }
    }

    future::join_all(tasks).await;
    Ok(())
}
