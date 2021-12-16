use crate::repos::config::Config;
use std::cmp::Ordering;
use super::package::{ControlFile, DebPackage};
use super::cache;

// use solvent;
// use solvent::DepGraph;

// TODO: I really need to make this better
fn get_version(dep: &str) -> Option<(Ordering, &str)> {
    if dep.contains("(") && dep.contains(")") {
        let start = match dep.find("=") {
            Some(v) => v+1,
            None => dep.find('(').unwrap()+1
        };
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

// Remove the version naming from the package
fn parse_name(dep: &str) -> &str {
    let dep = dep.trim();
    if dep.contains("(") && dep.contains(")") {
        let end = dep.find("(").unwrap();
        dep[..end].trim()
    } else {
        dep
    }
}

pub fn get_dependencies(control: &ControlFile, list: &mut Vec<ControlFile>) {
    if let Some(dependencies) = &control.depends {
        for dep in dependencies.iter() {
            list.push(dep.clone());
            get_dependencies(&dep, list);
        }
    }
}

// TODO: When done, verify at opm's database
fn check_dependencie(config: &mut Config, dependencie: &str) -> Option<DebPackage> {
    // println!("Parsed name: -{}-", parse_name(dependencie));
    let dep = cache::db_lookup(config, parse_name(dependencie), true, false).unwrap();
    if let Some(dep) = dep.into_iter().next() {
        if let Some(version) = get_version(dependencie) {
            if deb_version::compare_versions(&dep.control.version, &version.1) != version.0 {
                println!("Need version {} of {}", version.1, dep.control.package);
                return None;
            }
        }
        Some(dep)
    } else {
        if let Some(curr_version) = get_version(dependencie) {
             println!("Need to install {} version {}", dependencie, curr_version.1);
        }
        None
    }
}

pub fn parse_dependencies(config: &mut Config, dependencies: Option<Vec<String>>) -> Option<Vec<ControlFile>> {
    let mut deps = Vec::new();
    
    if let Some(dependencies) = dependencies {
        dependencies.into_iter()
            .for_each(|pkg_name| {
                println!("Name: {}", pkg_name);
                if pkg_name.contains("|") {
                    let var = pkg_name.split(" | ").map(|name| name.to_owned()).collect::<Vec<_>>(); 
                    println!("You need to install one of them after: {:?}", var);
                } else {
                    if let Some(_) = check_dependencie(config, &pkg_name) {
                        println!("OK {}", pkg_name);
                        // deps.push(pkg.0.control);
                    } else {
                        if let Some(pkg) = cache::cache_lookup(config, &pkg_name) {
                            println!("NEED TO INSTALL {}", pkg_name);

                            deps.push(pkg.control);
                        }
                    }
                }
            });
            Some(deps)
    } else {
        None
    }
}