use std::cmp::Ordering;
use anyhow::Result;
use solvent::DepGraph;

use crate::repos::{config::Config, errors::InstallError};
use super::package::ControlFile;
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

pub fn get_dependencies(config: &Config, pkg: ControlFile, deps: Option<Vec<String>>, depgraph: &mut DepGraph<Option<ControlFile>>, force: bool) -> Result<()> {
    if let Some(deps) = deps {
        if !deps.is_empty() {
            for name in deps.iter()
            .flat_map(|name| parse_name(name).trim().split(" | "))
            .filter(|name| cache::check_installed(config, name).is_none())
            {                
                if let Some(deb) = cache::cache_lookup(config, name)? {
                    if !force {
                        match (deb.control.breaks.clone(), deb.control.conflicts.clone()) {
                            (Some(b), Some(c)) => {
                                check_if_breaks(config, &b)?;
                                check_if_breaks(config, &c)?;
                            },
                            (Some(b), None) => check_if_breaks(config, &b)?,
                            (None, Some(c)) => check_if_breaks(config, &c)?,
                            (None, None) => (),
                        }
                    }
                    
                    if let Some(version) = get_version(&deb.control.version) {
                        if !check_version(version, &pkg.version) {
                            anyhow::bail!(InstallError::Error(format!("Version {} ({}) is not satisfied! Need version {} ({})", deb.control.version, deb.control.package, pkg.version, pkg.package)));
                        }
                    }
                    
                    if depgraph.dependencies_of(&Some(deb.control.clone())).is_err() {
                        depgraph.register_dependency(Some(pkg.clone()), Some(deb.control.clone()));
    
                        if deb.control.depends.is_some() {
                            get_dependencies(config, deb.control.clone(), deb.control.depends, depgraph, force)?;
                        }
                    }

                } else {
                    anyhow::bail!(InstallError::Error("No package was found ...".to_owned()));
                }
            }
        } else {
            depgraph.register_dependency(Some(pkg), None);    
        }
        Ok(())
    } else {
        depgraph.register_dependency(Some(pkg), None);
        Ok(())
    }
}

fn check_if_breaks(config: &Config, pkgs: &[String]) -> Result<()> {
    if pkgs.iter()
        .flat_map(|name| parse_name(name).trim().split(" | "))
        .filter(|name| cache::check_installed(config, name).is_some())
        .count() > 0 {
        anyhow::bail!(InstallError::Breaks("The package you're trying to install will break/conflict with others".to_owned()));
    } else {
        Ok(())
    }
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
        assert!(get_dependencies(&config, Some("accountsservice")).is_some());
    }
}