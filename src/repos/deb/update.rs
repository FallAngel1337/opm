use xz2::read::XzDecoder;

use reqwest;
use std::{io::{self, ErrorKind, prelude::*}, thread::LocalKey};
use std::fs::{self, File};
use std::str;

use super::sources::DebianSource;
use crate::repos::{errors::InstallError, deb::package::{ControlFile, DebPackage, PkgKind}, database::PackageStatus};
use crate::repos::config::Config;

use crate::repos::cache as opm_cache;
use super::cache as deb_cache;

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
pub async fn update(config: &mut Config, repos: &Vec<DebianSource>) -> Result<(), InstallError> {
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

// async fn update_cache(config: &mut Config, repos: &Vec<DebianSource>) -> Result<(), InstallError> {
//     for (i, source) in repos.iter().enumerate() {
//         println!("Get {}: {} {} {:?}", i+1, source.url, source.distribution, source.components);
//         for perm in source.components.iter() {
//             let pkgcache = format!("{}dists/{}/{}/binary-amd64/Packages.xz", source.url, source.distribution, perm); // Binary packages ONLY for now
            
//             let response = reqwest::get(pkgcache).await?;
            
//             let content = response.bytes().await?;
//             let content: &[u8] = content.as_ref();
            
//             let mut data = XzDecoder::new(content);
//             let mut bytes = Vec::new();
//             data.read_to_end(&mut bytes).unwrap_or_default();
//             let bytes: &[u8] = bytes.as_ref();
            
//             let url = str::replace(&source.url, "http://", "");
//             let url = str::replace(&url, "/", "_");

//             let repo = format!("{}{}_{}_binary-amd64_Packages", url, source.distribution, perm);
//             // println!("Package name: {}", repo);
            
//             let url = repo
// 			.split("/")
// 			.last()
// 			.unwrap()
// 			.replace("_", "/")
// 			.split("/")
// 			.next()
// 			.unwrap()
// 			.to_owned();
            
//             let control = str::from_utf8(bytes).expect("Invalid content");
            
//             let control = control
//                 .split("\n\n")
//                 .map(|ctrl| ControlFile::from(config, ctrl))
//                 .map(|mut ctrl| {
//                     let url = format!("http://{}/ubuntu/{}", url, ctrl.filename);
//                     ctrl.set_filename(&url);
//                     ctrl
//                 })
//                 .map(|ctrl| {
//                  DebPackage {
//                     control: ctrl,
//                     kind: PkgKind::Binary,
//                     signature: "NOPE".to_owned(),
//                     status: PackageStatus::Installed
//                 }
//             }).collect::<Vec<_>>();

//             opm_cache::load_into(config, control)?;

//             // println!("")


//             // println!("URL: {}", url);
//             // println!("Donwloaded: {:?}", control[0]);

            
//             // io::copy(&mut bytes, &mut pkg)?;
//         };
//     }

//     Ok(())
// }

async fn update_releases(config: &Config, repos: &Vec<DebianSource>) -> Result<(), InstallError> {
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