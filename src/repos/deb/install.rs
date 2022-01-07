use anyhow::{self, Result};
use std::{path::Path, io::Write};
use std::time::Instant;

///
/// Debian package install
///

use crate::repos::{errors::InstallError, deb::package::{DebPackage, PkgKind}};
use crate::repos::config::Config;
use bytesize::ByteSize;
use super::{extract, download};
use super::{cache, scripts};
use futures::future;
use fs_extra;

// TODO: Check for newer versions of the package if installed
pub async fn install(config: &Config, name: &str) -> Result<()> {
    crate::repos::lock::lock()?;

    if name.ends_with(".deb") {
        let pkg = extract::extract(config, name)?;
        let (pkg, info) = (pkg.0, pkg.1);

        if let Some(pkg) = cache::check_installed(config, &pkg.control.package) {
            println!("{} - {}", pkg.control.package, pkg.control.version);
            anyhow::bail!(InstallError::AlreadyInstalled);
        }

        scripts::execute_install_pre(&info)?;
        scripts::execute_install_pos(&info)?;
    } else {
        if let Some(pkg) = cache::check_installed(config, name) {
            println!("{} - {}", pkg.control.package, pkg.control.version);
            anyhow::bail!(InstallError::AlreadyInstalled);
        }

        println!("Installing {} for debian ...", name);
        println!("Looking up for dependencies ...");
        if let Some(mut pkg) = cache::cache_lookup_deps(config, name)? {
            println!("Done");
            let mut tasks = vec![];
            let mut new_packages = vec![pkg.control.clone()];

            println!("Found {:?}", pkg.control.package);

            if let Some(dependencies) = &mut pkg.control.depends {
                new_packages.append(dependencies);
            }

            println!("Installing {} NEW package", new_packages.len());
            let mut total = 0;
            new_packages.iter().for_each(|pkg| {
                println!(" {}", pkg.package);
                total += pkg.size.parse::<u64>().unwrap();
            });

            println!("After this operation, {} of additional disk space will be used.", ByteSize(total));            
            print!("Do you want to continue? [Y/n] ");
            let mut answer = String::new();
            
            std::io::stdout().flush()?;
            std::io::stdin().read_line(&mut answer)?;
            
            let answer = answer.to_ascii_lowercase().trim().chars().next().unwrap();
            if answer != 'y' {
                eprintln!("Exiting installation process...");
                anyhow::bail!(InstallError::Interrupted);
            }

            for control in new_packages.iter() {
                tasks.push(download::download(config, DebPackage { control: control.clone(), kind: PkgKind::Binary }));
            }
            
            let start = Instant::now();
            for data in future::join_all(tasks).await {
                let path = data?;

                let path = path
                    .into_os_string()
                    .into_string().unwrap();
                
                let pkg = extract::extract(config, &path)?;
                let (pkg, info) = (pkg.0, pkg.1);

                println!("Installing {} ...", pkg.control.package);    
                scripts::execute_install_pre(&info)?;
                scripts::execute_install_pos(&info)?;
                finish(Path::new(&config.tmp), &pkg.control.package)?;
                cache::add_package(config, pkg)?;
            }
            let duration = start.elapsed();
            println!("Installed {} in {}s", name, duration.as_secs());


        } else {
            anyhow::bail!(InstallError::NotFoundError(name.to_string()));
        }
    }

    Ok(())
}

fn finish(p: &Path, name: &str) -> Result<()> {
    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;
    
    match fs_extra::dir::copy(&p, std::path::Path::new("/"), &options) {
        Ok(_) => (),
        Err(e) => if let fs_extra::error::ErrorKind::NotFound = e.kind { 
            anyhow::bail!(InstallError::BrokenPackage(name.to_owned()))
        } else {
            panic!("{}", e);
        }
    }
    
    fs_extra::dir::remove(&p)?;
    Ok(())
}

