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

// TODO: Check for newer versions of the package if installed
pub fn install(config: &mut Config, name: &str) -> Result<()> {
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
        
        if let Some(pkg) = cache::cache_lookup(config, name) {
            let mut new_packages = vec![pkg.clone()];

            println!("Found {:?}", pkg.control.package);
            if let Some(dep) = dependencies::get_dependencies(config, &pkg) {
                let deps = dep.0;
                let sugg = dep.1;

                new_packages.append(&mut deps.clone());

                println!("Installing {} NEW packages", new_packages.len());
                new_packages.iter().for_each(|pkg| print!("{} ", pkg.control.package));
                println!();

                if !sugg.is_empty() {
                    println!("Suggested packages:");
                    sugg.iter().for_each(|pkg| print!("{} ", pkg));
                    println!();
                }
                
                for pkg in deps.into_iter() {
                    if let Ok(path) = download::download(config, &pkg) {
                        let path = path
                            .into_os_string()
                            .into_string().unwrap();
                        
                        let pkg = extract::extract(config, &path, &pkg.control.package)?;

                        println!("Installing {} ...", pkg.control.package);
                        scripts::execute_install(&config.tmp)?;
                    } else {
                        println!("Could not download {}", pkg.control.package);
                    }
                }
            }

            let path = download::download(config, &pkg).unwrap();
            let path = path
                .into_os_string()
                .into_string().unwrap();
            
            let pkg = extract::extract(config, &path, name)?;
            println!("Installing {} ...", pkg.control.package);

            scripts::execute_install(&config.tmp)?;
            finish(&config.tmp)?;

        } else {
            anyhow::bail!(InstallError::NotFoundError(name.to_string()));
        }
    }
    
    Ok(())
}

fn finish(p: &str) -> Result<(), InstallError> {
    let _options = fs_extra::dir::CopyOptions::new();
    let mut vec = Vec::new();
    let p = Path::new(p);

    for entry in std::fs::read_dir(&p).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            vec.push(path);
        }
    }

    // fs_extra::copy_items(&vec, std::path::Path::new("/"), &options).unwrap();
    // fs_extra::remove_items(&vec).unwrap();

    Ok(())
}