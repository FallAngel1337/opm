///
/// Debian package install
/// 

use crate::repos::errors::InstallError;
use crate::repos::config::Config;
use super::package::ControlFile;
use super::extract;
use super::download;
use std::fs;

fn cache_lookup(config: &Config, name: &str) {
    for entry in fs::read_dir(&config.cache).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        let control = fs::read_to_string(path)
            .unwrap()
            .split("\n\n")
            .map(|ctrl| ControlFile::from(ctrl).unwrap())
            .filter(|ctrl| ctrl.package.contains(name))
            .collect::<Vec<_>>();
            // .count();

        let entry = entry.path()
            .into_os_string()
            .into_string()
            .unwrap();

        let url =  entry
            .split("/")
            .last()
            .unwrap()
            .replace("_", "/")
            .split("/")
            .next()
            .unwrap()
            .to_owned();

            
        // if control.len() > 0 { println!("Found {} package entries for {} at {}", control.len(), name, url) } else { () }
        for pkg in control {
            let url = format!("{}/ubuntu/{}", url, pkg.filename);
            println!("{} {} - {} ({})", pkg.package, pkg.version, pkg.description, url);
        }
    };
}

pub fn install(config: &mut Config, name: &str) -> Result<(), InstallError> {
    if name.ends_with(".deb") {
        let _pkg = extract::extract(name, &config.tmp)?;
        println!("Extracting ...");
    } else {
        download::download(config, name)?;
    }

    cache_lookup(&config, name);
    println!("Installing {}", name);

    Ok(())
}