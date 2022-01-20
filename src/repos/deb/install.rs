use indicatif::{HumanDuration, HumanBytes, MultiProgress,ProgressBar, ProgressStyle};
use anyhow::{self, Result};
use solvent::DepGraph;
use std::{path::Path, io::Write};
use std::time::Instant;

///
/// Debian package install
///

use crate::repos::{errors::InstallError, deb::{package::{DebPackage, PkgKind}, dependencies::get_dependencies}};
use crate::repos::config::Config;
use super::{extract, download};
use super::{cache, scripts};
use futures::future;
use fs_extra;

// TODO: Check for newer versions of the package if installed
// TODO: Get rid of most of those `clone()` calls
pub async fn install(config: &Config, name: &str, force: bool) -> Result<()> {
    crate::repos::lock::lock()?;

    if name.ends_with(".deb") {
        let pkg = extract::extract(config, name, name.split(".deb").next().unwrap())?;
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

        if let Some(pkg) = cache::cache_lookup(config, name)? {
            println!("Done");
            let mut depgraph = DepGraph::new();
            let mut tasks = vec![];

            get_dependencies(config, pkg.control.clone(), pkg.control.clone().depends, &mut depgraph, force)?;

            let pkgs = depgraph.dependencies_of(&Some(pkg.control)).unwrap()
                .filter_map(|node| node.ok())
                .flatten().cloned()
                .collect::<Vec<_>>();

            println!("Installing {} NEW package", pkgs.len());

            let mut total = 0;
            for pkg in pkgs.iter() {
                print!(" {}", pkg.package);
                total += pkg.size.parse::<u64>().unwrap();
            }
            println!();

            println!("After this operation, {} of additional disk space will be used.", HumanBytes(total));
            print!("Do you want to continue? [Y/n] ");
            let mut answer = String::new();
            std::io::stdout().flush()?;
            std::io::stdin().read_line(&mut answer)?;
            let answer = answer.to_ascii_lowercase().trim().chars().next().unwrap();

            if answer != 'y' {
                eprintln!("Exiting installation process...");
                anyhow::bail!(InstallError::Interrupted);
            }

            let mp = MultiProgress::new();
            for pkg in pkgs.clone().into_iter() {
                let bar = mp.add(ProgressBar::new(pkg.size.parse::<u64>()?));
                bar.set_style(ProgressStyle::default_bar()
                    .template(" [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
                    .progress_chars("#>-"));

                tasks.push(download::download(config, DebPackage { control: pkg, kind: PkgKind::Binary }, bar));
            }
            let handle = tokio::task::spawn_blocking(move || mp.join().unwrap());
            
            let start = Instant::now();
            for data in future::join_all(tasks).await
                .into_iter()
                .filter_map(|r| r.ok())
                .zip(pkgs.into_iter().map(|ctrl| ctrl.package)) 
            {
                let (path, name) = data;

                let path = path
                    .into_os_string()
                    .into_string().unwrap();
                
                let pkg = extract::extract(config, &path, &name)?;
                let (pkg, info, data) = (pkg.0, pkg.1, pkg.2);

                println!("Installing {} ...", pkg.control.package);    
                scripts::execute_install_pre(&info)?;
                scripts::execute_install_pos(&info)?;
                finish(Path::new(&data.control_path), &pkg.control.package)?;
                cache::add_package(config, pkg)?;
            }
            let duration = start.elapsed();
            println!("Installed {} in {}s", name, HumanDuration(duration));

            handle.await?;

        } else {
            anyhow::bail!(InstallError::NotFoundError(name.to_string()));
        }
    }


    Ok(())
}

fn finish(p: &Path, name: &str) -> Result<()> {
    use fs_extra::error::ErrorKind;
    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;
    
    for path in std::fs::read_dir(p)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
    {
        match fs_extra::dir::copy(&path, std::path::Path::new("/"), &options) {
            Ok(_) => (),
            Err(e) => match e.kind { 
                ErrorKind::NotFound => anyhow::bail!(InstallError::BrokenPackage(name.to_owned())),
                ErrorKind::InvalidFolder | ErrorKind::AlreadyExists => continue,
                // _ => eprintln!("{} -> {:?}", e, path)
                _ => panic!("{} -> {:?}", e, path)
            }
        }
    }

    Ok(())
}

