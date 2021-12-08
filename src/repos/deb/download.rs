// use super::config::DebianSource;
use crate::repos::errors::InstallError;
use crate::repos::config::Config;
use crate::repos::cache as rpm_cache;
use std::cmp::Ordering;
use super::cache;
use deb_version;

// TODO: I really need to make this better
fn get_version(dep: &str) -> Option<(Ordering, &str)> {
    if dep.contains("(") && dep.contains(")") {
        let start = dep.find("=").unwrap()+1;
        let end = dep.find(")").unwrap();
        
        let version = dep[start..end].trim();

        if dep.contains("=") && !(dep.contains("<") || dep.contains(">")) {
            Some((Ordering::Equal, version))
        } else if version.contains(">") {
            Some((Ordering::Greater, version))
        } else {
            Some((Ordering::Less, version))
        }

    } else {
        None
    }
}

fn check_dependencies(dependencies: &Vec<String>) -> bool {
    dependencies.iter()
    .for_each(|elem| {
        if let Some(dep) = cache::dpkg_cache_lookup(elem, true) {
            if let Some(dep_version) = get_version(&elem) {
                let ok = deb_version::compare_versions(dep_version.1, &dep.version);
                if dep_version.0 != ok {
                    println!("Required version: {:?}", dep_version);
                }
            }
        } else {
            if let Some(dep_version) = get_version(&elem) {
                println!("NEED TO BE INSTALLED {} : {}", elem, dep_version.1);
            } else {
                println!("NEED TO INSTALL {}", elem);
            }
        }

        if elem.contains("|") {
            let dep = elem.split("|")
                .map(|e| e.to_owned())
                .collect();
            check_dependencies(&dep);
        }
    });

    true
}

#[tokio::main]
pub async fn download(config: &Config, name: &str) -> Result<(), InstallError> {
    println!("Downloading {} from {:?}", name, config.cache);

    match rpm_cache::cache_lookup(config, name, true) {
        Some(v) => {
            v.iter().for_each(|pkg| {
                println!("Found {} {}", pkg.0.package, pkg.0.version);
                check_dependencies(&pkg.0.depends);
            });
        },

        None => println!("Package {} not found", name)
    };

    Ok(())
}