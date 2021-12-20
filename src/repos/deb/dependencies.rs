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
                let pkgs = pkg.split(" | ")
                .collect::<Vec<_>>();
                
                let installed = pkgs.iter()
                    .filter_map(|pkg| cache::check_installed(pkg))
                    .count();

                if installed == 0 {
                    // NOTE: If none is installed, install the first one
                    let pkgs = pkgs.into_iter()
                    .filter_map(|pkg| cache::cache_lookup(config, pkg))
                    .next();

                    if let Some(pkg) = pkgs {
                        depends.push(pkg)
                    } else {
                        return None;
                    }
                }
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