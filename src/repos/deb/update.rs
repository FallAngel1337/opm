use xz::read::XzDecoder;
// use bytes::Buf;

use reqwest;
use std::io::{self, ErrorKind, prelude::*};
use std::fs::{self, File, OpenOptions};
use std::str;

use super::config::DebianSource;
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
    match fs::remove_dir_all(&config.pkgs){
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
    fs::create_dir(&config.pkgs)?;
    fs::create_dir(&config.tmp)?;

    Ok(())
}

pub async fn update(config: &Config, repos: &Vec<DebianSource>) -> Result<(), InstallError> {
    clear(config)?;

    // update_releases(config, repos).await?;
    update_cache(config, repos).await?;

    Ok(())
}

async fn update_cache(config: &Config, repos: &Vec<DebianSource>) -> Result<(), InstallError> {
    for (i, source) in repos.iter().enumerate() {
        println!("Get {}: {} {} {:?}", i+1, source.url, source.distribution, source.components);
        for perm in source.components.iter() {
            let pkgcache = format!("{}dists/{}/{}/binary-amd64/Packages.xz", source.url, source.distribution, perm); // Binary packages ONLY for now
            
            let response = reqwest::get(pkgcache).await?;

            let mut xzencoded = {
                let fname = response
                    .url()
                    .path_segments()
                    .and_then(|segments| segments.last())
                    .and_then(|name| if name.is_empty() { None } else { Some(name) })
                    .unwrap_or("tmp.bin");
        
                println!("file to download: '{}'", fname);
                let url = str::replace(&source.url, "http://", "");
                let url = str::replace(&url, "/", "_");
                
                let pkg = config.cache.join(format!("{}{}_{}_binary-amd64_Packages.xz", url, source.distribution, perm));
                println!("PKG = {:?}", pkg);
                let file = match OpenOptions::new()
                    .read(true).write(true)
                    .create(true).open(&pkg) {
                        Ok(f) => f,
                        Err(e) => panic!("oh no {}", e)
                    };
                file
            };

            let content =  response.bytes().await?;
            let mut content: &[u8] = content.as_ref();
            println!("Length: {}", content.len());
            io::copy(&mut content, &mut xzencoded)?;
            
            let url = str::replace(&source.url, "http://", "");
            let url = str::replace(&url, "/", "_");
            
            let pkg = config.cache.join(format!("{}{}_{}_binary-amd64_Packages", url, source.distribution, perm));
            println!("Saving to: {:?}", pkg);
            println!("encoded=> {:?}", xzencoded);
            let mut pkg = File::create(pkg)?;
            println!("len = {}", content.len());
            let mut bytes = [0 as u8; 1000000]; // = Vec::new();
            let mut data = XzDecoder::new(xzencoded);
            // data.read_to_end(&mut bytes);
            data.read(&mut bytes);

            println!("data => {}", bytes.len());
            
            let mut bytes: &[u8] = bytes.as_ref();
            io::copy(&mut bytes, &mut pkg)?;
        };
    }

    Ok(())
}

async fn update_releases(config: &Config, repos: &Vec<DebianSource>) -> Result<(), InstallError> {
    for (_i, source) in repos.iter().enumerate() {
        for perm in source.components.iter() {
            let release_file = format!("{}dists/{}/InRelease", source.url, source.distribution);
            
            let response = reqwest::get(release_file).await?;
            
            let mut dest = {
                let fname = response
                    .url()
                    .path_segments()
                    .and_then(|segments| segments.last())
                    .and_then(|name| if name.is_empty() { None } else { Some(name) })
                    .unwrap_or("tmp.bin");
        
                println!("file to download: '{}'", fname);
                let url = str::replace(&source.url, "http://", "");
                let url = str::replace(&url, "/", "_");
                
                let dir = config.cache.join(format!("{}{}_{}_binary-amd64_{}", url, source.distribution, perm, fname));
                // println!("RLS = {:?}", dir);
                match fs::create_dir(&dir) {
                    Ok(_) => (),
                    Err(e) => match e.kind() {
                        ErrorKind::AlreadyExists => (),
                        _ => panic!("fuck {:?} => {}", dir, e)
                    }
                }
                let fname = dir.join(&fname);
                // println!("will be located under: '{:?}'", fname);
                File::create(fname)?
            };

            let content =  response.text().await?;
            io::copy(&mut content.as_bytes(), &mut dest)?;
        }

    }
    Ok(())
}