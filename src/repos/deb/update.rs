use anyhow::Result;
use xz2::read::XzDecoder;
use std::{
    io::{ErrorKind, prelude::*},
    fs::{self, File},
    path::Path,
    str
};
use reqwest;
use sha2::{Sha256, Digest};
use futures::future;
use super::sources::DebianSource;
use crate::repos::config::Config;

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
    match tokio::join!(
        update_releases(config, repos),
        update_cache(config, repos)
    )
    {
        (Ok(()), Ok(())) => Ok(()),
        _ => panic!("Something gone wrong when updating")
    }
}

async fn update_cache(config: &Config, repos: &[DebianSource]) -> Result<()> {
    let mut tasks = vec![];
    for (i, source) in repos.iter().enumerate() {
        for perm in source.components.iter() {
            let pkgcache = format!("{}dists/{}/{}/binary-amd64/Packages.xz", source.url, source.distribution, perm); // Binary packages ONLY for now

            let url = str::replace(&source.url, "http://", "");
            let url = str::replace(&url, "/", "_");

            let pkg = Path::new(&config.cache).join(format!("{}{}_{}_binary-amd64_Packages", url, source.distribution, perm));
            
            let mut data = Vec::new();
            if let Ok(mut file) = File::open(&pkg) {
                file.read_to_end(&mut data)?;
            }

            let mut old_hash = Sha256::new();
            old_hash.update(data);
            let old_hash = old_hash.finalize();
            
            tasks.push(tokio::spawn(async move {
                let response = reqwest::get(&pkgcache).await.unwrap();
                
                let content = response.bytes().await.unwrap();
                let content: &[u8] = content.as_ref();

                println!("HIT {}: {} [{} kB]", i+1, pkgcache, content.len() / 1024);
                
                let mut data = XzDecoder::new(content);
                let mut bytes = Vec::new();
                
                data.read_to_end(&mut bytes).unwrap_or_default();
                
                let mut bytes: &[u8] = bytes.as_ref();
                
                let mut new_hash = Sha256::new();
                new_hash.update(bytes);
                let new_hash = new_hash.finalize();
                
                if old_hash != new_hash {
                    let mut pkg = tokio::fs::File::create(pkg).await.unwrap();
                    tokio::io::copy(&mut bytes, &mut pkg).await.unwrap();
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
            
            let rls = Path::new(&config.rls).join(format!("{}{}_{}_binary-amd64_InRelease", url, source.distribution, perm));
            
            let mut data = Vec::new();
            if let Ok(mut file) = File::open(&rls) {
                file.read_to_end(&mut data)?;
            }

            let mut old_hash = Sha256::new();
            old_hash.update(&data);
            let old_hash = old_hash.finalize();
            
            tasks.push(tokio::spawn(async move {
                let response = reqwest::get(&release_file).await.unwrap();
                let content =  response.text().await.unwrap();
                println!("HIT {}: {} [{} kB]", i+1, release_file, content.len() / 1024);
                
                let mut new_hash = Sha256::new();
                new_hash.update(content.as_bytes());
                let new_hash = new_hash.finalize();
                
                if old_hash != new_hash {
                    let mut dest = tokio::fs::File::create(rls).await.unwrap();
                    tokio::io::copy(&mut content.as_bytes(), &mut dest).await.unwrap();
                }

            }));
        }
    }

    future::join_all(tasks).await;
    Ok(())
}
