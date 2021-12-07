use crate::repos::config::Config;
use super::package::ControlFile;
use std::fs;

pub fn cache_lookup(config: &Config, name: &str) -> Vec<(ControlFile, String)> {
    let mut pkgs = Vec::new();

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
        control.into_iter().for_each(|pkg| {
            let url = format!("{}/ubuntu/{}", url, &pkg.filename);
            pkgs.push((pkg, String::from(&url)));
        });
    }

    pkgs
}