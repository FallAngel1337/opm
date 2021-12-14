use crate::repos::config::Config;
use super::package::ControlFile;
use std::fs;

///
/// Dump all installed packages from /var/lib/dpkg/status
/// 
pub fn dpkg_cache_dump(config: &Config) -> Vec<ControlFile> {
	let control = fs::read_to_string("/var/lib/dpkg/status").unwrap();

	let control = control
		.split("\n\n")
		.map(|ctrl| ControlFile::from(config, ctrl))
		.collect::<Vec<_>>();
	
	control
}