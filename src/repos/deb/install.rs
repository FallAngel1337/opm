use anyhow::{self, Result};
use std::path::Path;

///
/// Debian package install
///

use crate::repos::{errors::InstallError, deb::package::{DebPackage, PkgKind}};
use crate::repos::config::Config;
use super::{extract, download};
use super::{cache, scripts};
use futures::future;
use fs_extra;

// TODO: Check for newer versions of the package if installed
pub async fn install(config: &Config, name: &str) -> Result<()> {
    // crate::repos::lock::lock()?;

    if name.ends_with(".deb") {
        let pkg_name = name.rsplit(".deb").next().unwrap();
        let pkg = extract::extract(config, name, pkg_name)?;
        let (pkg, info) = (pkg.0, pkg.1);

        if let Some(pkg) = cache::check_installed(config, &pkg.control.package) {
            println!("{} - {}", pkg.control.package, pkg.control.version);
            anyhow::bail!(InstallError::AlreadyInstalled);
        }

        scripts::execute_install(&info)?;
    } else {
        if let Some(pkg) = cache::check_installed(config, name) {
            println!("{} - {}", pkg.control.package, pkg.control.version);
            anyhow::bail!(InstallError::AlreadyInstalled);
        }

        println!("Installing {} for debian ...", name);
        println!("Looking up for dependencies ...");
        if let Some(pkg) = cache::cache_lookup_deps(config, name)? {
            println!("Done");
            let mut new_packages = 1;
            let mut tasks = vec![];

            println!("Found {:?}", pkg.control.package);

            if let Some(dependencies) = &pkg.control.depends {
               for control in dependencies.iter() {
                    tasks.push(download::download(config, DebPackage { control: control.clone(), kind: PkgKind::Binary }));
                    new_packages += 1;
                }
            }

            tasks.push(download::download(config, pkg));

            println!("Installing {} NEW package", new_packages);

            for data in future::join_all(tasks).await {
                let data = data?;
                let (path, pkg_name) = data;

                let path = path
                    .into_os_string()
                    .into_string().unwrap();
                    
                let pkg = extract::extract(config, &path, &pkg_name)?;
                let (pkg, info) = (pkg.0, pkg.1);

                println!("Checking the signatures ...");

                println!("Installing {} ...", pkg_name);
                let path = format!("{}/{}", config.tmp, pkg_name);
                scripts::execute_install(&info)?;
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

    for entry in std::fs::read_dir(&p)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // fs_extra::dir::create_all(&path, false).unwrap();
            match fs_extra::dir::copy(&path, std::path::Path::new("/"), &options) {
                Ok(_) => (),
                Err(e) => match e.kind {
                    fs_extra::error::ErrorKind::AlreadyExists => (),
                    fs_extra::error::ErrorKind::NotFound => (),
                    _ => panic!("Some error occurred :: {:?} - {}", path, e)
		        }
            };
            fs_extra::dir::remove(&path).unwrap();
        }
    }
     
    Ok(())
}
