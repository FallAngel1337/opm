use anyhow::{self, Result};
use std::path::Path;

///
/// Debian package install
///

use fs_extra;
use crate::repos::{errors::InstallError, deb::dependencies};
use crate::repos::config::Config;
use super::cache;
use super::{extract, download};
use super::scripts;
use futures::future;

// TODO: Check for newer versions of the package if installed
#[tokio::main]
pub async fn install(config: &Config, name: &str) -> Result<()> {
    if name.ends_with(".deb") {
        let pkg_name = name.rsplit(".deb").next().unwrap();
        let pkg = extract::extract(config, name, pkg_name)?;

        if let Some(pkg) = cache::check_installed(&pkg.control.package) {
            println!("{} - {}", pkg.control.package, pkg.control.version);
            anyhow::bail!(InstallError::AlreadyInstalled);
        }

        scripts::execute_install(&config.tmp)?;
    } else {
        if let Some(pkg) = cache::check_installed(name) {
            println!("{} - {}", pkg.control.package, pkg.control.version);
            anyhow::bail!(InstallError::AlreadyInstalled);
        }

        println!("Downloading {} for debian ...", name);

        if let Some(pkg) = cache::cache_lookup(config, name)? {
            let mut new_packages = vec![pkg.clone()];

            println!("Found {:?}", pkg.control.package);
            if let Some(dep) = dependencies::get_dependencies(config, &pkg) {
                let (deps, sugg) = dep;
                
                new_packages.append(&mut deps.clone());

                println!("Installing {} NEW packages", new_packages.len());
                new_packages.iter().for_each(|pkg| print!("{} ", pkg.control.package));
                println!();

                if !sugg.is_empty() {
                    println!("Suggested packages:");
                    sugg.iter().for_each(|pkg| print!("{} ", pkg));
                    println!();
                }

                let mut tasks = vec![];
                for pkg in deps.iter() {
                    tasks.push(download::download(config, pkg));
                }

                for path in future::join_all(tasks).await {
                    let path = path?
                        .into_os_string()
                        .into_string().unwrap();

                    let pkg = extract::extract(config, &path, &pkg.control.package)?;

                    println!("Installing {} ...", pkg.control.package);
                    scripts::execute_install(&config.tmp)?;
                    finish(Path::new(&format!("{}/{}", config.tmp, pkg.control.package)))?;
                }
            }

            // let path = path
            //     .into_os_string()
            //     .into_string().unwrap();
            
            // let pkg = extract::extract(config, &path, &pkg.control.package)?;

            // println!("Installing {} ...", pkg.control.package);
            // scripts::execute_install(config.tmp)?;
            // finish(Path::new(&format!("{}/{}", config.tmp, pkg.control.package)))?;
            
            let path = download::download(config, &pkg).await?;
            let path = path
                .into_os_string()
                .into_string().unwrap();

            let pkg = extract::extract(config, &path, name)?;
            println!("Installing {} ...", pkg.control.package);

            scripts::execute_install(&config.tmp)?;
            finish(Path::new(&config.tmp))?;

        } else {
            anyhow::bail!(InstallError::NotFoundError(name.to_string()));
        }
    }

    Ok(())
}

fn finish(p: &Path) -> Result<()> {
    let options = fs_extra::dir::CopyOptions::new();
    let mut items = vec![];

    for entry in std::fs::read_dir(&p).unwrap() {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            items.push(path);
        }
    }
    
    fs_extra::copy_items(&items, std::path::Path::new("/"), &options).unwrap();
    fs_extra::remove_items(&items).unwrap();
    Ok(())
}
