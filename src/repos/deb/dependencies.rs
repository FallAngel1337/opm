use crate::repos::cache as opm_cache;
use crate::repos::config::Config;
use std::cmp::Ordering;
use super::package::ControlFile;
use super::cache;

// use solvent;
// use solvent::DepGraph;

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

// Remove the version naming from the package
fn parse_name(dep: &str) -> &str {
    if dep.contains("(") && dep.contains(")") {
        let end = dep.find("(").unwrap();
        dep[..end].trim()
    } else {
        dep
    }
}

// TODO: When done, verify at opm's database
fn check_dependencie(config: &Config, dependencie: &str) -> bool {
    
    if dependencie.contains("|") {
        let dep = dependencie.split("|")
        .map(|e| e.trim())
        .collect::<Vec<_>>();
        
        for e in dep {
            if !check_dependencie(config, e) {
                println!("HERE >> {}", e);
                return false;
            }
        }
    }
    
    if let Some(dep) = cache::dpkg_cache_lookup(config, parse_name(dependencie), true) {
        let dep = dep.into_iter().next().unwrap();
        println!("package: {}\nversion: {}", dep.package, dep.version);
        if let Some(curr_version) = get_version(dependencie) {
            if deb_version::compare_versions(&dep.version, curr_version.1) != curr_version.0 {
                println!("Need version {} of {}", curr_version.1, dep.package);
                return false;
            }
        }
        true
    } else {
        if let Some(curr_version) = get_version(dependencie) {
             println!("Need to install {} version {}", dependencie, curr_version.1);
        }
        false
    }
}

fn get_dependencies(control: &ControlFile, list: &mut Vec<ControlFile>) {
    if let Some(dependencies) = &control.depends {
        for dep in dependencies.iter() {
            list.push(dep.clone()); // TODO: do not use clone
            get_dependencies(dep, list);
        }
    }
}

pub fn parse_dependencies(config: &Config, dependencies: Option<Vec<String>>) -> Option<Vec<ControlFile>> {
    let mut deps = Vec::new();
    // println!("=> {:?}", dependencies);
    if let Some(dependencies) = dependencies {
        dependencies.into_iter()
            .for_each(|pkg_name| {
                // println!("{}", pkg_name);
            });
            // println!(">> {:?}", deps);
            Some(deps)
    } else {
        None
    }
}

pub fn solve_dependencies(config: &Config, dependencies: &ControlFile) -> Vec<ControlFile> {
    vec![]
}