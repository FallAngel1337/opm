use anyhow::{self, Result};
use std::path::Path;

///
/// Debian package install
///

use crate::repos::{errors::InstallError, deb::dependencies};
use crate::repos::config::Config;
use super::{extract, download};
use super::{cache, scripts, signatures};
use futures::future;
use fs_extra;

// TODO: Check for newer versions of the package if installed
pub async fn install(config: &Config, name: &str) -> Result<()> {
    if name.ends_with(".deb") {
        let pkg_name = name.rsplit(".deb").next().unwrap();
        let pkg = extract::extract(config, name, pkg_name)?;

        if let Some(pkg) = cache::check_installed(config, &pkg.control.package) {
            println!("{} - {}", pkg.control.package, pkg.control.version);
            anyhow::bail!(InstallError::AlreadyInstalled);
        }

        scripts::execute_install(&config.tmp)?;
    } else {
        if let Some(pkg) = cache::check_installed(config, name) {
            println!("{} - {}", pkg.control.package, pkg.control.version);
            anyhow::bail!(InstallError::AlreadyInstalled);
        }

        println!("Downloading {} for debian ...", name);

        if let Some(pkg) = cache::cache_lookup(config, name)? {
            let mut new_packages = vec![pkg.clone()];
            let mut tasks = vec![];

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

                for pkg in deps.into_iter() {
                    tasks.push(download::download(config, pkg));
                }
            }

            tasks.push(download::download(config, pkg));

            for data in future::join_all(tasks).await {
                let data = data?;
                let (path, pkg_name) = data;

                let path = path
                    .into_os_string()
                    .into_string().unwrap();

                    
                let pkg = extract::extract(config, &path, &pkg_name)?;
                
                println!("Checking the signatures ...");
                if !signatures::verify_sig(&pkg, Path::new(&path))? {
                    println!("Packages signatures did not match ...");
                } else {
                    println!("OK");
                }

                println!("Installing {} ...", pkg_name);
                let path = format!("{}/{}", config.tmp, pkg_name);
                scripts::execute_install(&config.info)?;
                finish(Path::new(&path))?;
                cache::add_package(config, pkg)?;
            }
        } else {
            anyhow::bail!(InstallError::NotFoundError(name.to_string()));
        }
    }

    Ok(())
}

fn finish(p: &Path) -> Result<()> {
    let options = fs_extra::dir::CopyOptions::new();
    let mut items = vec![];

    for entry in std::fs::read_dir(&p)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            items.push(path);
        }
    }
    
    fs_extra::copy_items(&items, std::path::Path::new("/"), &options)?;
    fs_extra::remove_items(&items)?;

    Ok(())
}
