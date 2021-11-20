use reqwest;
use std::io::{self, ErrorKind};
use std::fs::{self, File};
use std::str;

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
                let url = str::replace(&source.url, "http://", "");
                let url = str::replace(&url, "/", "_");
                
                let dir = config.cache.join(format!("{}_{}_{}_binary-amd64_{}", url, source.distribution, perm, fname));
                // let dir = config.cache.join(&source.distribution);
                match fs::create_dir(&dir) {
                    Ok(_) => (),
                    Err(e) => match e.kind() {
                        ErrorKind::AlreadyExists => (),
                        _ => panic!("fuck {}", e)
                    }
                }
                let fname = dir.join(&fname);
                println!("will be located under: '{:?}'", fname);
                File::create(fname)?
            };

            let content =  response.text().await?;
            io::copy(&mut content.as_bytes(), &mut dest)?;

            let response = reqwest::get(pkgcache).await?;
            
            let mut dest = {
                let fname = response
                    .url()
                    .path_segments()
                    .and_then(|segments| segments.last())
                    .and_then(|name| if name.is_empty() { None } else { Some(name) })
                    .unwrap_or("tmp.bin");

                // format!("{}_{}_{}_binary-amd64_{}", source.url, source.distribution, perm, fname)
                let url = str::replace(&source.url, "http://", "");
                let url = str::replace(&url, "/", "_");

                let dir = config.cache.join(format!("{}_{}_{}_binary-amd64_{}", url, source.distribution, perm, fname));
                println!("DIR = {:?}", dir);
                match fs::create_dir(&dir) {
                    Ok(_) => (),
                    Err(e) => match e.kind() {
                        ErrorKind::AlreadyExists => (),
                        _ => panic!("fuck {}", e)
                    }
                }
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