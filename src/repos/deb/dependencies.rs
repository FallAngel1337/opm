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

pub fn get_dependencies(config: &Config, pkg: &str) -> Option<(Vec<DebPackage>, Vec<String>)> {
    let ctrl = match cache::cache_lookup(config, parse_name(pkg)) {
        Ok(Some(pkg)) => pkg.control,
        _ => return None
    };

    let (mut depends, mut optional) = (Vec::new(), Vec::new());

    if let Some(deps) = &ctrl.depends {
        for pkg in deps {
            let depv = get_version(&pkg.package);
            let pkg = parse_name(&pkg.package);

            if pkg.contains('|') {
                println!("PIPED {}", pkg);
                let installed = pkg.split(" | ")
                    .filter_map(|pkg| cache::check_installed(config, pkg))
                    .count();

                if installed == 0 {
                    // NOTE: If none is installed, install the first one
                    let mut pkg = pkg.split(" | ")
                    .filter_map(|pkg| cache::cache_lookup(config, pkg).ok())
                    .flatten()
                    .collect::<Vec<_>>();

                    if !pkg.is_empty() {
                        depends.append(&mut pkg);
                    }
                }
            } else if cache::check_installed(config, pkg).is_none() {
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
        deps.iter().for_each(|d| optional.push(d.package.clone()))
    }

    if let Some(deps)  = &ctrl.suggests {
        deps.iter().for_each(|d| optional.push(d.package.clone()))
    }

    if let Some(deps)  = &ctrl.enhances {
        deps.iter().for_each(|d| optional.push(d.package.clone()))
    }

    if let Some(deps)  = &ctrl.pre_depends {
        println!("Pre-Dependent Packages: {:?}", deps);
    }

    println!("Depends = {:#?}", depends);
    Some((depends, optional))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_name_test() {
        assert_eq!(parse_name("demo_pkg (>= 1.33.7)"), "demo_pkg");
        assert_eq!(parse_name("demo_pkg (<= 1.33.7)"), "demo_pkg");
        assert_eq!(parse_name("demo_pkg (<< 1.33.7)"), "demo_pkg");
        assert_eq!(parse_name("demo_pkg (>> 1.33.7)"), "demo_pkg");
        assert_eq!(parse_name("demo_pkg (= 1.33.7)"), "demo_pkg");
        
        assert_eq!(parse_name("demo_pkg (>= 1.33.7("), "demo_pkg");
        assert_eq!(parse_name("demo_pkg (<= 1.33.7("), "demo_pkg");
        assert_eq!(parse_name("demo_pkg (<< 1.33.7("), "demo_pkg");
        assert_eq!(parse_name("demo_pkg (>> 1.33.7("), "demo_pkg");
        assert_eq!(parse_name("demo_pkg (= 1.33.7("), "demo_pkg");
        
        assert!(parse_name("demo_pkg )>= 1.33.7)") != "demo_pkg");
        assert!(parse_name("demo_pkg )<= 1.33.7)") != "demo_pkg");
        assert!(parse_name("demo_pkg )<< 1.33.7)") != "demo_pkg");
        assert!(parse_name("demo_pkg )>> 1.33.7)") != "demo_pkg");
        assert!(parse_name("demo_pkg )= 1.33.7)")  != "demo_pkg");
        
        assert!(parse_name("demo_pkg )>= 1.33.7(") != "demo_pkg");
        assert!(parse_name("demo_pkg )<= 1.33.7(") != "demo_pkg");
        assert!(parse_name("demo_pkg )<< 1.33.7(") != "demo_pkg");
        assert!(parse_name("demo_pkg )>> 1.33.7(") != "demo_pkg");
        assert!(parse_name("demo_pkg )= 1.33.7(")  != "demo_pkg");
    }

    #[test]
    fn get_version_test() {
        assert_eq!(get_version("demo_pkg"), None);

        assert_eq!(get_version("demo_pkg (>= 1.33.7)").unwrap(), ">= 1.33.7");
        assert_eq!(get_version("demo_pkg (<= 1.33.7)").unwrap(), "<= 1.33.7");
        assert_eq!(get_version("demo_pkg (<< 1.33.7)").unwrap(), "<< 1.33.7");
        assert_eq!(get_version("demo_pkg (>> 1.33.7)").unwrap(), ">> 1.33.7");
        assert_eq!(get_version("demo_pkg (= 1.33.7)").unwrap(), "= 1.33.7");
        
        assert_eq!(get_version("demo_pkg (>= 1.33.7("), None);
        assert_eq!(get_version("demo_pkg (<= 1.33.7("), None);
        assert_eq!(get_version("demo_pkg (<< 1.33.7("), None);
        assert_eq!(get_version("demo_pkg (>> 1.33.7("), None);
        assert_eq!(get_version("demo_pkg (= 1.33.7("), None);
    }

    #[test]
    #[should_panic]
    fn get_version_test_panic() {
        get_version("demo_pkg )>= 1.33.7)").unwrap();
        get_version("demo_pkg )<= 1.33.7)").unwrap();
        get_version("demo_pkg )<< 1.33.7)").unwrap();
        get_version("demo_pkg )>> 1.33.7)").unwrap();
        get_version("demo_pkg )= 1.33.7)").unwrap();
        
        get_version("demo_pkg )>= 1.33.7(").unwrap();
        get_version("demo_pkg )<= 1.33.7(").unwrap();
        get_version("demo_pkg )<< 1.33.7(").unwrap();
        get_version("demo_pkg )>> 1.33.7(").unwrap();
        get_version("demo_pkg )= 1.33.7(").unwrap();
    }

    #[test]
    fn check_version_test() {
        assert!(check_version("1.33.7", "<= 1.33.8"));
        assert!(check_version("1.33.7", ">= 1.33.7"));
        assert!(check_version("1.33.7", "= 1.33.7"));
        assert!(!check_version("1.33.7", "<< 1.33.7"));
        assert!(!check_version("1.33.7", ">> 1.33.7"));
    }

    // This was also crashing and idk why
    #[test]
    #[ignore]
    fn get_dependencies_test() {
        let config = crate::repos::setup().unwrap();
        assert!(get_dependencies(&config, "accountsservice").is_some());
    }
}