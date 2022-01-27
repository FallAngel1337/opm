use indicatif::{HumanBytes, MultiProgress,ProgressBar, ProgressStyle};
use anyhow::{self, Result};
use solvent::DepGraph;
use std::{path::Path, io::Write};

///
/// Debian package install
///

use crate::repos::{errors::{InstallError, CacheError}, deb::{package::{DebPackage, PkgKind}, dependencies::get_dependencies}};
use crate::repos::config::Config;
use super::{extract, download};
use super::{cache, scripts};
use futures::future;
use async_recursion::async_recursion;

// TODO: Check for newer versions of the package if installed
// TODO: Get rid of most of those `clone()` calls
#[async_recursion]
pub async fn install(config: &Config, name: &str, force: bool) -> Result<()> {
    if name.ends_with(".deb") {
        let pkg = extract::extract(config, name, name.split(".deb").next().unwrap())?;
        let (pkg, info, data) = (pkg.0, pkg.1, pkg.2);

        if let Some(pkg) = cache::check_installed(config, &pkg.control.package) {
            println!("{} - {}", pkg.control.package, pkg.control.version);
            anyhow::bail!(InstallError::AlreadyInstalled(pkg.control.package));
        }

        println!("Installing {} ...", pkg.control.package);
        scripts::execute_install_pre(&info)?;
        scripts::execute_install_pos(&info)?;

        finish(Path::new(&data.control_path)).unwrap();
        cache::add_package(config, pkg)?;
    } else {
        if let Some(pkg) = cache::check_installed(config, name) {
            println!("{} - {}", pkg.control.package, pkg.control.version);
            anyhow::bail!(InstallError::AlreadyInstalled(pkg.control.package));
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
                anyhow::bail!(InstallError::UserInterrupt);
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

            for data in future::join_all(tasks).await
                .into_iter()
                .filter_map(|r| r.ok())
                .zip(pkgs.into_iter().map(|ctrl| ctrl.package)) 
            {
                let (path, _name) = data;
                install(config, path.to_str().unwrap(), force).await?;
            }
            fs_extra::dir::create(&config.tmp, true)?;
            handle.await?;
        } else {
            anyhow::bail!(CacheError::NotFoundError { pkg: name.to_owned(), cache: config.cache.clone() });
        }
    }

    Ok(())
}

fn finish(p: &Path) -> Result<()> {
    use fs_extra::error::ErrorKind;
    let options = fs_extra::dir::CopyOptions::new();

    for path in std::fs::read_dir(p)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
    {
        match fs_extra::dir::copy(&path, std::path::Path::new("/"), &options) {
            Ok(_) => (),
            Err(e) => match e.kind { 
                ErrorKind::InvalidFolder | ErrorKind::AlreadyExists | ErrorKind::NotFound => continue,
                _ => panic!("{} -> {:?}", e, path)
            }
        }
    }

    Ok(())
}