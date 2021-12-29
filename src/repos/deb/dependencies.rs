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
            let depv = get_version(pkg);
            let pkg = parse_name(pkg);

            if pkg.contains('|') {
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

#[cfg(test)]
mod test {
    use crate::repos::{self, deb::package::{ControlFile, PkgKind}};
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
        let config = repos::setup().unwrap();
        let data = r"Package: accountsservice
        Architecture: amd64
        Version: 0.6.55-0ubuntu11
        Priority: standard
        Section: gnome
        Origin: Ubuntu
        Maintainer: Ubuntu Developers <ubuntu-devel-discuss@lists.ubuntu.com>
        Original-Maintainer: Debian freedesktop.org maintainers <pkg-freedesktop-maintainers@lists.alioth.debian.org>
        Bugs: https://bugs.launchpad.net/ubuntu/+filebug
        Installed-Size: 452
        Depends: dbus, libaccountsservice0 (= 0.6.55-0ubuntu11), libc6 (>= 2.4), libglib2.0-0 (>= 2.44), libpolkit-gobject-1-0 (>= 0.99)
        Suggests: gnome-control-center
        Filename: pool/main/a/accountsservice/accountsservice_0.6.55-0ubuntu11_amd64.deb
        Size: 60940
        MD5sum: 87a0e27c83950d864d901ceca0f2b49c
        SHA1: ce92ea3783ca4ca6cdb5115381379f9c1317566b
        SHA256: e34884d71bb98002bf0c775479aa31ee5011ded1abf969ffe6496874de499f42
        Homepage: https://www.freedesktop.org/wiki/Software/AccountsService/
        Description: query and manipulate user account information
        Task: standard
        Description-md5: 8aeed0a03c7cd494f0c4b8d977483d7e";
        let res = get_dependencies(&config, &DebPackage {
            control: ControlFile::from(data).unwrap(),
            kind: PkgKind::Binary
        });

        dbg!("=> {:?}", res);
        assert!(2 == 3);
    }
}