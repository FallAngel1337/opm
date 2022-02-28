use crate::repos::{
    errors::{
        InstallError,
        CacheError
    },
    deb::{
        package::{
            DebPackage,
            PkgKind
        },
        dependencies::get_dependencies
    },
    config::Config,
};
use super::{
    extract::extract,
    download::download,
};
use super::{
    cache,
    scripts,
};
use indicatif::{
    HumanBytes,
    MultiProgress,
    ProgressBar,
    ProgressStyle,
    HumanDuration,
};
use anyhow::{self, Result};
use solvent::DepGraph;
use tokio::time::Instant;
use std::{path::Path, io::{self, Write}};

///
/// Debian package install
///
use futures::future;

fn user_input() -> Result<()> {
    let mut answer = String::new();
    io::stdout().flush()?;
    io::stdin().read_line(&mut answer)?;

    if answer.to_ascii_lowercase().trim().starts_with('y') {
        Ok(())
    } else {
        eprintln!("Exiting installation process...");
        anyhow::bail!(InstallError::UserInterrupt);
    }
}

// TODO: Get rid of most of those `clone()` calls
pub async fn install(config: &Config, name: &str, force: bool, dest: Option<String>) -> Result<()> {
    if name.ends_with(".deb") {
        let pkg = extract(config, name, name.rsplit('/').next().unwrap().split(".deb").next().unwrap())?;
        let (pkg, info) = (pkg.0, pkg.1);

        if let Some(pkg) = cache::check_installed(config, &pkg.control.package) {
            println!("{} - {}", pkg.control.package, pkg.control.version);
            anyhow::bail!(InstallError::AlreadyInstalled(pkg.control.package));
        }

        println!("Installing {} ...", pkg.control.package);
        scripts::execute_install_pre(&info)?;
        scripts::execute_install_pos(&info)?;

        finish(config, pkg, &dest)?;
    } else {
        // TODO: Find out a better way of checking for new packages
        if let Some(pkg) = cache::check_installed(config, name) {
            println!("{} - {}", pkg.control.package, pkg.control.version);
            let new = cache::cache_lookup(config, &pkg.control.package)?.unwrap();
            if new.control.version != pkg.control.version {
                println!("A new version is available\nOld: {:#?} | New: {:#?}", pkg.control.version, new.control.version);
                print!("Want to update? [Y/n] ");
                user_input()?;
            } else {
                anyhow::bail!(InstallError::AlreadyInstalled(pkg.control.package));
            }
        }

        println!("Installing {} for debian ...", name);
        println!("Looking up for dependencies ...");

        if let Some(pkg) = cache::cache_lookup(config, name)? {
            println!("Done");
            let mut depgraph = DepGraph::new();
            let mut tasks = vec![];

            depgraph.register_dependency(Some(pkg.control.clone()), None);
            get_dependencies(config, pkg.control.clone(), pkg.control.clone().depends, &mut depgraph, force)?;

            let pkgs = depgraph.dependencies_of(&Some(pkg.control)).unwrap()
                .filter_map(|node| node.ok())
                .flatten().cloned()
                .collect::<Vec<_>>();

            println!("Installing {} NEW package", pkgs.len());

            let mut total = 0;
            pkgs.iter().for_each(|pkg| {
                print!(" {}", pkg.package);
                total += pkg.size.parse::<u64>().unwrap();
            });

            println!("\nAfter this operation, {} of additional disk space will be used.", HumanBytes(total));
            print!("Do you want to continue? [Y/n] ");
            user_input()?;

            let mp = MultiProgress::new();
            for pkg in pkgs.clone().into_iter() {
                let bar = mp.add(ProgressBar::new(pkg.size.parse::<u64>()?));
                bar.set_style(ProgressStyle::default_bar()
                    .template(" [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
                    .progress_chars("#>-"));

                tasks.push(download(config, DebPackage { control: pkg, kind: PkgKind::Binary }, bar));
            }
            let handle = tokio::task::spawn_blocking(move || mp.join().unwrap());

            let start = Instant::now();
            for data in future::join_all(tasks).await
                .into_iter()
                .filter_map(|r| r.ok())
                .zip(pkgs.into_iter().map(|ctrl| ctrl.package)) 
            {
                let (path, _) = data;
                let pkg = extract(config, path.to_str().unwrap(), name)?.0;
                finish(config, pkg, &dest)?;
            }
            println!("Installed {} in {}", name, HumanDuration(start.elapsed()));
            handle.await?;
        } else {
            anyhow::bail!(CacheError::NotFoundError { pkg: name.to_owned(), cache: config.cache.clone() });
        }
    }

    Ok(())
}

fn finish(config: &Config, pkg: DebPackage, dest: &Option<String>) -> Result<()> {
    use fs_extra::error::ErrorKind;
    let mut options = fs_extra::dir::CopyOptions::new();
    options.skip_exist = true;
    let mut register_package = false;

    let dest = match dest {
        Some(dest) => {
            let dest = Path::new(dest);
            if !dest.exists() {
                fs_extra::dir::create_all(dest, false)?;
            }
            dest
        },
        None => {
            register_package = true;
            Path::new("/")
        },
    };

    for path in std::fs::read_dir(&config.tmp)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
    {
        match fs_extra::dir::copy(&path, dest, &options) {
            Ok(_) => (),
            Err(e) => match e.kind { 
                ErrorKind::InvalidFolder | ErrorKind::AlreadyExists => continue,
                _ => panic!("{} -> {:?}", e, path)
            }
        }
    }

    if register_package {
        cache::add_package(config, pkg)?;
    }

    Ok(())
}