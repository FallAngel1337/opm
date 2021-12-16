use super::cache as opm_cache;
use super::config::Config;
use super::utils::PackageFormat;

pub fn search(config: &mut Config, name: &str) {
    if let Some(_) = PackageFormat::get_format() {
        let _pkgs = opm_cache::lookup(config, name, false, true);
	} else {
        eprintln!("Consider define `PKG_FMT` environment variable!");
        std::process::exit(1);
    }
}