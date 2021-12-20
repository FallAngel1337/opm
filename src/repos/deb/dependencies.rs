use crate::repos::config::Config;
use super::package::DebPackage;
use super::cache;

fn parse_name(name: &str) -> &str {
    if name.contains('(') {
        let end = name.find('(').unwrap() - 1;
        &name[..end]
    } else {
        name
    }
}

pub fn get_dependencies(config: &Config, pkg: &DebPackage) -> Option<Vec<DebPackage>> {
    let ctrl = &pkg.control;
    println!("Getting dependencies for \"{}\" ...", ctrl.package);

    let mut depends = Vec::new();
    
    if let Some(deps) = &ctrl.depends {
        for pkg in deps {
            // println!("Before: -{}-", pkg);
            let pkg = parse_name(&pkg);
            // println!("After: -{}-", pkg);

            if pkg.contains('|') {
                println!("Found alternative package names: {:?}", pkg);
                pkg.split(" | ")
                    .filter_map(|pkg| cache::cache_lookup(config, pkg))
                    .for_each(|pkg| {
                    if let Some(mut found) = get_dependencies(config, &pkg) {
                        depends.append(&mut found);
                    }
                });

            } else if cache::check_installed(pkg).is_none() {
                if let Some(pkg) = cache::cache_lookup(config, pkg) {
                    // get_dependencies(config, &pkg);
                    // println!("Got {:?} from cache!", pkg.control.package);
                    depends.push(pkg);
                } else {
                    return None;
                }
            }
        }
        depends.dedup();
        // println!("Depends = {:#?}", depends);
    }
    
    if let Some(deps) = &ctrl.recommends {
        println!("Recommendded Packages: {:?}", deps);
    }

    if let Some(deps)  = &ctrl.suggests {
        println!("Suggested Packages: {:?}", deps);
    }

    if let Some(deps)  = &ctrl.enhances {
        println!("Enhancement Packages: {:?}", deps);
    }

    if let Some(deps)  = &ctrl.pre_depends {
        println!("Pre-Dependent Packages: {:?}", deps);
    }

    Some(depends)
}