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

        for pkg in control {
            println!("Package: {}", pkg.package);
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