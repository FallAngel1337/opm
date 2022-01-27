use anyhow::Result;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle, HumanBytes};
use xz2::read::XzDecoder;
use flate2::read::GzDecoder;
use std::{
    io::{ErrorKind, prelude::*},
    fs,
    path::Path,
    str
};
use futures::{future, StreamExt};
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
    let (mut cache, mut rls) = (vec![], vec![]);
    let spinner_style = ProgressStyle::default_spinner()
        .template("{spinner} {prefix}");

    let mp = MultiProgress::new();
    for (i, source) in repos.iter().enumerate() {
        for perm in source.components.iter() {
            let cache_bar = mp.add(ProgressBar::new(0));
            cache_bar.set_style(spinner_style.clone());

            let rls_bar = mp.add(ProgressBar::new(0));
            rls_bar.set_style(spinner_style.clone());
            
            cache.push(update_cache(config, &source.url, &source.distribution, perm, cache_bar, i));
            rls.push(update_releases(config, &source.url, &source.distribution, perm, rls_bar, i));
        }
    }
    let handle = tokio::task::spawn_blocking(move || mp.join().unwrap());

    future::join_all(rls).await;
    future::join_all(cache).await;
    
    handle.await?;

    Ok(())
}

async fn update_cache(config: &Config, url: &str, dist: &str, perm: &str, pb: ProgressBar, counter: usize) -> Result<()> {
    // Binary packages ONLY for now
    let pkgcache = format!("{}dists/{}/{}/binary-amd64/Packages.xz", url, dist, perm);
    let response = match reqwest::get(&pkgcache).await {
        Ok(r) => Some(r),
        Err(_) => {
            let pkgcache = format!("{}dists/{}/{}/binary-amd64/Packages.gz", url, dist, perm);
            match reqwest::get(&pkgcache).await {
                Ok(r) => Some(r),
                Err(e) => {
                    eprintln!("Could not get the package at {} due {}", pkgcache, e);
                    None
                }
            }
        }
    };

    let url = str::replace(url, "http://", "");
    let url = str::replace(&url, "/", "_");    
    
    if let Some(response) = response {
        let size = response.content_length().unwrap_or_default();
        pb.set_length(size);
        pb.set_prefix(format!("{}: {} [{}]", counter+1, pkgcache, HumanBytes(size)));

        let (mut stream, mut downloaded) = (response.bytes_stream(), 0u64);
        
        let mut content = Vec::with_capacity(size as usize);
        let pkg = Path::new(&config.cache).join(format!("{}dists_{}_{}_binary-amd64_Packages", url, dist, perm));

        while let Some(item) = stream.next().await {
            let chunk = item?;
            Write::write(&mut content, &chunk)?;
            let progress = std::cmp::min(downloaded + chunk.len() as u64, size);
            downloaded = progress;
            pb.set_position(progress);
        }
        
        let mut bytes = Vec::new();
        unpack(&pkgcache, content.as_ref(), &mut bytes);
        let mut bytes: &[u8] = bytes.as_ref();
        if !bytes.is_empty() {
            let mut pkg = tokio::fs::File::create(pkg).await.unwrap();
            tokio::io::copy(&mut bytes, &mut pkg).await.unwrap();
        }

        pb.finish_and_clear()
    }
    
    Ok(())
}

async fn update_releases(config: &Config, url: &str, dist: &str, perm: &str, pb: ProgressBar, counter: usize) -> Result<()> {

    let release_file = format!("{}dists/{}/InRelease", url, dist);

    let url = str::replace(url, "http://", "");
    let url = str::replace(&url, "/", "_");
    
    let response = reqwest::get(&release_file).await?;
    let size = response.content_length().unwrap_or_default();
    pb.set_length(size);
    pb.set_prefix(format!("{}: {} [{}]", counter+1, release_file, HumanBytes(size)));
    
    let (mut stream, mut downloaded) = (response.bytes_stream(), 0u64);

    let mut content = Vec::with_capacity(size as usize);
    let rls = Path::new(&config.rls).join(format!("{}dists_{}_{}_binary-amd64_InRelease", url, dist, perm));

    while let Some(item) = stream.next().await {
        let chunk = item?;
        Write::write(&mut content, &chunk)?;
        let progress = std::cmp::min(downloaded + chunk.len() as u64, size);
        downloaded = progress;
        pb.set_position(progress);
    }
    
    
    let mut dest = tokio::fs::File::create(rls).await.unwrap();
    tokio::io::copy(&mut content.as_ref(), &mut dest).await.unwrap();

    pb.finish_and_clear();
    Ok(())
}
