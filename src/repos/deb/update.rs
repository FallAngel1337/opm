use anyhow::Result;
use xz2::read::XzDecoder;
use std::{
    io::{ErrorKind, prelude::*},
    fs::{self, File},
    path::Path,
    str, collections::HashMap
};
use reqwest;
use futures::future;
use super::sources::DebianSource;
use crate::repos::config::Config;

#[derive(Debug)]
struct Release {
    origin: String,
    label: String,
    suite: String,
    version: String,
    codename: String,
    date: String,
    architectures: Vec<String>,
    components: Vec<String>,
    description: String,
    md5sum: Vec<String>
}

impl Release {
    fn new(file: &Path) -> Result<Self> {
        let mut map: HashMap<Option<String>, Option<String>> = HashMap::new();
        let contents = std::fs::read_to_string(file)?;

        for line in contents.lines() {
            let line = line.trim();
            let values = line.splitn(2, ':').map(|line| line.to_owned()).collect::<Vec<_>>();
            map.insert(
                values.get(0).map(|v| v.to_owned()),
                values.get(1).map(|v| v.to_owned())
            );
        }

        Ok(
            Self {
                origin: Self::try_get(&map, "Origin")?,
                label: Self::try_get(&map, "Label")?,
                suite: Self::try_get(&map, "Suite")?,
                version: Self::try_get(&map, "Version")?,
                codename: Self::try_get(&map, "Codename")?,
                description: Self::try_get(&map, "Description")?,
                date: Self::try_get(&map, "Date")?,
                architectures: Self::split_deps(Some(&Self::try_get(&map, "Architectures").unwrap_or_default())).unwrap(),
                components: Self::split_deps(Some(&Self::try_get(&map, "Components").unwrap_or_default())).unwrap(),
                md5sum: Self::split_deps(Some(&Self::try_get(&map, "Components").unwrap_or_default())).unwrap(),
            }
        )
    }

    fn try_get(hashmap: &HashMap<Option<String>, Option<String>>, field: &str) -> Result<String> {
        let value = hashmap.get(&Some(field.to_owned()));
        if let Some(v) = value {
            if let Some(v) = v {
                Ok (v.trim().to_owned())
            } else {
                anyhow::bail!(format!("Unknown error trying to get \"{}\" field", field));
            }
        } else {
            anyhow::bail!(format!("Invalid debain's control file! Missing \"{}\" field", field));
        }
    }

    fn split_deps(dependencies: Option<&String>) -> Option<Vec<String>> {
        if let Some(val) = dependencies {
            if !val.is_empty() {
                let val = val
                    .split(',')
                    .map(|d| d.trim().to_owned())
                    .collect::<Vec<_>>();
                Some(val)
            } else {
                None
            }
        } else {
            None
        }
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
            let pkgcache_xz = format!("{}dists/{}/{}/binary-amd64/Packages.xz", source.url, source.distribution, perm);
            let pkgcache_gz = format!("{}dists/{}/{}/binary-amd64/Packages.gz", source.url, source.distribution, perm);

            let url = str::replace(&source.url, "http://", "");
            let url = str::replace(&url, "/", "_");

            let pkg = Path::new(&config.cache).join(format!("{}dists_{}_{}_binary-amd64_Packages", url, source.distribution, perm));
            let rsl = format!("{}dists_{}_{}_binary-amd64_Packages", url, source.distribution, perm);

            let old_size = if let Ok(mut file) = File::open(&pkg) {
                let mut old_data = Vec::new();
                file.read_to_end(&mut old_data)?
            } else {
                0
            };
            
            tasks.push(tokio::spawn(async move {
                let (response_xz, response_gz) = (reqwest::get(&pkgcache_xz).await, reqwest::get(&pkgcache_gz).await);
                
                let content = if let Ok(res) = response_xz { (&pkgcache_xz, res) } else { (&pkgcache_gz, response_gz.unwrap()) };
                let pkgcache = content.0;
                let content = content.1.bytes().await.unwrap();

                println!("Hit {}: {} [{} kB]", i+1, pkgcache, content.len() / 1024);
                
                let mut data = XzDecoder::new(content.as_ref());
                let mut bytes = Vec::new();
                
                data.read_to_end(&mut bytes).unwrap_or_default();
                
                let mut bytes: &[u8] = bytes.as_ref();
                
                if bytes.len() != old_size {
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
            
            let rls = Path::new(&config.rls).join(format!("{}dists_{}_{}_binary-amd64_InRelease", url, source.distribution, perm));
            
            let old_size = if let Ok(mut file) = File::open(&rls) {
                let mut old_data = Vec::new();
                file.read_to_end(&mut old_data)?
            } else {
                0
            };
            
            tasks.push(tokio::spawn(async move {
                let response = reqwest::get(&release_file).await.unwrap();
                let content =  response.text().await.unwrap();
                println!("Hit {}: {} [{} kB]", i+1, release_file, content.len() / 1024);
                
                if content.len() != old_size {
                    let mut dest = tokio::fs::File::create(rls).await.unwrap();
                    tokio::io::copy(&mut content.as_bytes(), &mut dest).await.unwrap();
                    
                }

            }));
        }
    }

    future::join_all(tasks).await;
    Ok(())
}
