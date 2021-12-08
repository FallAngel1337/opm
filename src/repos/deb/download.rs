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

fn check_dependencies(dependencie: &str) -> bool {
    if dependencie.contains("|") {
        let dep = dependencie.split("|")
            .map(|e| e.trim())
            .collect::<Vec<_>>();
        
        for e in dep {
            if !check_dependencies(e) {
                println!("HERE >> {}", e);
                return false;
            }
        }
    }

    if let Some(dep) = cache::dpkg_cache_lookup(dependencie, true) {
        if let Some(dep_version) = get_version(dependencie) {
            if deb_version::compare_versions(&dep.version, dep_version.1) != dep_version.0 {
                println!("Need version {} of {}", dep_version.1, dep.package);
            }
        }
        true
    } else {
        if let Some(dep_version) = get_version(dependencie) {
             println!("Need to install {} version {}", dependencie, dep_version.1);
        }
        false
    }
}

#[tokio::main]
pub async fn download(config: &Config, name: &str) -> Result<(), InstallError> {
    let mut need_to_install = Vec::new();
    println!("Downloading {} from {:?}", name, config.cache);
    
    match rpm_cache::cache_lookup(config, name, true) {
        Some(v) => {
            v.iter().for_each(|pkg| {
                println!("Found {} {}", pkg.0.package, pkg.0.version);
                pkg.0.depends.clone().into_iter().for_each(|dependencie| {
                    if !check_dependencies(&dependencie) {
                        need_to_install.push(dependencie);
                    }
                })
            });

            println!("Dependens on: {:?}", need_to_install);
        },

        None => println!("Package {} not found", name)
    };

    Ok(())
}