use std::cmp::Ordering;
use crate::repos::config::Config;
use super::package::DebPackage;
use super::cache;


fn parse_name(name: &str) -> &str {
    let end = name.find('(');
    match end {
        Some(e) => &name[..e-1],
        None => name
    }
}

fn get_version(pkg: &str) -> Option<&str> {
    let (start, end) = (pkg.find('('), pkg.find(')'));
    if let (Some(start), Some(end)) = (start, end) {
        Some(&pkg[start+1..end])
    } else {
        None
    }
}

fn check_version(pkgv: &str, depv: &str) -> bool {
    use Ordering::{Equal, Greater, Less};
    
    let full_version = depv.split(' ').collect::<Vec<_>>();
    let sig = full_version.get(0).unwrap();
    let number = full_version.get(1).unwrap();
    let result = deb_version::compare_versions(pkgv, number);
    
    // Debugging message
    // println!("Full: {:?}\nSignal: {}\nNumber: {}\nResult: {:?}", full_version, sig, number, result);

    match *sig {
        "=" => result == Equal,
        ">>" => result == Greater,
        "<<" => result == Less,
        ">=" => result == Greater || result == Equal,
        "<=" => result == Less || result == Equal,
        _ => false
    }
}

pub fn get_dependencies(config: &Config, pkg: &DebPackage) -> Option<(Vec<DebPackage>, Vec<String>)> {
    let ctrl = &pkg.control;

    let (mut depends, mut optional) = (Vec::new(), Vec::new());

    if let Some(deps) = &ctrl.depends {
        for pkg in deps {
            println!("Getting {} ...", pkg);
            let depv = get_version(pkg);
            let pkg = parse_name(pkg);

            if pkg.contains('|') {
                // Recursion was causing the process to be killed by the OOM
                // pkg.split(" | ")
                //     .filter_map(|pkg| cache::cache_lookup(config, pkg).unwrap())
                //     .for_each(|pkg| {
                //         if let Some(mut found) = get_dependencies(config, &pkg) {
                //             depends.append(&mut found.0);
                //         }
                //     });
                let pkgs = pkg.split(" | ")
                    .filter_map(cache::check_installed);

                if pkgs.clone().count() == 0 {
                    // NOTE: If none is installed, install the first one
                    let pkg = pkgs
                        .filter_map(|pkg| cache::cache_lookup(config, &pkg.control.package).unwrap())
                        .next().unwrap();

                    depends.push(pkg)
                }
            } else if cache::check_installed(pkg).is_none() {
                if let Some(pkg) = cache::cache_lookup(config, pkg).unwrap() {
                    let pkgv = &pkg.control.version;
                    if let Some(depv) = depv {
                        if !check_version(pkgv, depv) {
                            eprintln!("Version {} of {} package is not satisfied! Need version {} of {}", pkgv, pkg.control.package, pkg.control.package, depv);
                        }
                    }
                    depends.push(pkg);
                } else {
                    return None;
                }
            }
        }
        depends.dedup();
    }
    
    if let Some(deps) = &ctrl.recommends {
        optional.append(&mut deps.clone())
    }

    if let Some(deps)  = &ctrl.suggests {
        optional.append(&mut deps.clone())
    }

    if let Some(deps)  = &ctrl.enhances {
        optional.append(&mut deps.clone())
    }

    if let Some(deps)  = &ctrl.pre_depends {
        println!("Pre-Dependent Packages: {:?}", deps);
    }

    Some((depends, optional))
}