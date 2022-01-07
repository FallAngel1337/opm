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
    // crate::repos::lock::lock()?;

    if name.ends_with(".deb") {
        let pkg_name = name.rsplit(".deb").next().unwrap();
        let pkg = extract::extract(config, name, pkg_name)?;
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

            /*
            Reading package lists... Done
            Building dependency tree
            Reading state information... Done
            The following additional packages will be installed:
            X Y
            Suggested packages:
            A
            The following NEW packages will be installed:
            X Y
            0 upgraded, 0 newly installed, 0 to remove and 9 not upgraded.
            Need to get <size kB|MB> of archives.
            After this operation, <size kB|MB> of additional disk space will be used.
            Do you want to continue? [Y/n] y
            Get:1 <url> <arc> X Version [<size> kB|MB]
            Get:2 <url> <arc> Y Version [<size> kB|MB]
            Fetched <size> kB|MB in <time (secs/mins/hrs/...)> (<dspeedavg kB/s|MB/s>)
            Preparing to unpack X.deb ...
            Unpacking X:<arch> (Version) ...
            Preparing to unpack Y.deb
            Unpacking Y:<arch> (Version) ...
            Setting up X:<arch> (Version) ...
            Setting up Y:<arch> (Version) ...
            <GREEN_TICK> X was installed successfully!
            <GREEN_TICK> Y was installed successfully!
            */
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
                std::process::exit(1); // TODO: bail a custom error here
            }

            for control in new_packages.iter() {
                tasks.push(download::download(config, DebPackage { control: control.clone(), kind: PkgKind::Binary }));
            }
            
            let start = Instant::now();
            for data in future::join_all(tasks).await {
                let data = data?;
                let (path, pkg_name) = data;

                let path = path
                    .into_os_string()
                    .into_string().unwrap();
                
                let pkg = extract::extract(config, &path, &pkg_name)?;
                let (_pkg, info) = (pkg.0, pkg.1);

                println!("Checking the signatures ...");

                println!("Installing {} ...", pkg_name);
                let path = format!("{}/{}", config.tmp, pkg_name);
                scripts::execute_install_pre(&info)?;
                scripts::execute_install_pos(&info)?;
                finish(Path::new(&path))?;
                // cache::add_package(config, pkg)?;
            }
            let duration = start.elapsed();
            println!("Installed {} in {}s", name, duration.as_secs());


        } else {
            anyhow::bail!(InstallError::NotFoundError(name.to_string()));
        }
    }

    Ok(())
}

fn finish(p: &Path) -> Result<()> {
    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;

    for entry in std::fs::read_dir(&p)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
    {
        if entry.is_dir() {
            fs_extra::dir::copy(&entry, std::path::Path::new("/tmp/fake_root"), &options)?;
        }
    }
    
    fs_extra::dir::remove(&p)?;
    Ok(())
}

