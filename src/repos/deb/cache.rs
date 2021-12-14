use crate::repos::config::Config;
use super::package::ControlFile;
use std::fs;

pub fn cache_dump(config: &Config) -> Vec<ControlFile> {
	let mut pkgs = Vec::new();
	for entry in fs::read_dir(&config.cache).unwrap() {
		let entry = entry.unwrap();
		let path = entry.path();
		
		let control = fs::read_to_string(path).unwrap();

		let control = control
			.split("\n\n")
			.map(|ctrl| ControlFile::from(config, ctrl));
			
		let entry = entry.path()
			.into_os_string()
			.into_string()
			.unwrap();

		let url =  entry
			.split("/")
			.last()
			.unwrap()
			.replace("_", "/")
			.split("/")
			.next()
			.unwrap()
			.to_owned();

		control.into_iter().for_each(|mut pkg| {
			let url = format!("{}/ubuntu/{}", url, &pkg.filename);
			pkg.set_filename(&url);
			pkgs.push(pkg);
		});
	}

	pkgs
}

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