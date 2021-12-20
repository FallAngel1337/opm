use crate::repos::config::Config;
use super::package::DebPackage;
use super::cache;

// use solvent;
// use solvent::DepGraph;

pub fn get_dependencies(config: &mut Config, pkg: &DebPackage) -> Option<Vec<DebPackage>> {
    let crtl = &pkg.control;
    let mut depends = Vec::new();

    if let Some(deps) = &crtl.depends {
        for pkg in deps {
            if pkg.contains('|') {
                let list = pkg.split(" | ").map(|pkg| pkg.to_owned()).collect::<Vec<_>>();
                println!("Which one you want to install?\n{:?}\n>> ", list);
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read from stdin");

                let input = input.trim().parse::<usize>().expect("Invalid number") - 1;
                let pkg = list.get(input).unwrap_or_else(|| panic!("Could not get the {}nth option", input));

                if cache::check_installed(pkg).is_none() {
                    if let Some(pkg) = cache::cache_lookup(config, pkg) {
                        get_dependencies(config, &pkg);
                        depends.push(pkg);
                    } else {
                        return None;
                    }
                }
            } else if cache::check_installed(pkg).is_none() {
                if let Some(pkg) = cache::cache_lookup(config, pkg) {
                    get_dependencies(config, &pkg);
                    depends.push(pkg);
                } else {
                    return None;
                }
            }
        }
        Some(depends)
    } else {
        None
    }
}