use crate::repos::config::Config;
use super::package::DebPackage;
use super::cache;

pub fn get_dependencies(config: &Config, pkg: &DebPackage) -> Option<Vec<DebPackage>> {
    let ctrl = &pkg.control;
    let mut depends = Vec::new();

    if let Some(deps) = &ctrl.depends {
        for pkg in deps {
            if pkg.contains('|') {
                let pkgs = pkg.split(" | ")
                .collect::<Vec<_>>();

                let installed = pkgs.iter()
                    .filter_map(|pkg| cache::check_installed(pkg))
                    .count();

                if installed == 0 {
                    // NOTE: If none is installed, install the first one
                    let pkg = pkgs.into_iter()
                        .filter_map(|pkg| cache::cache_lookup(config, pkg))
                        .next().unwrap();
                    depends.push(pkg)
                }
            } else if cache::check_installed(pkg).is_none() {
                if let Some(pkg) = cache::cache_lookup(config, pkg) {
                    // get_dependencies(config, &pkg);
                    depends.push(pkg);
                } else {
                    return None;
                }
            }
        }
        depends.dedup();
        println!("Depends = {:#?}", depends);
    } else {
        return None;
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