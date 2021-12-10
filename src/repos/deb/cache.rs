use crate::repos::config::Config;
use super::package::ControlFile;
use std::fs;

///
/// Lookup into the dpkg cache (/var/lib/dpkg/status).
/// Will be used to check for dependencies.
/// Note: WAY more slow
/// 
// TODO: Improve it to be less slow
pub fn dpkg_cache_lookup(config: &Config, name: &str, exact_match: bool) -> Option<ControlFile> {
	let control = fs::read_to_string("/var/lib/dpkg/status").unwrap();

	let control = control
		.split("\n\n")
		.map(|ctrl| ControlFile::from(config, ctrl).unwrap());
	
	let control = if exact_match {
		control
		.filter(|ctrl| ctrl.package == name)
		.collect::<Vec<_>>()
	} else {
		control
		.filter(|ctrl| ctrl.package.contains(name))
		.collect::<Vec<_>>()
	};

	if control.len() > 0 { 
		// println!("Found {} package entries for {}", control.len(), name);
		Some(control[0].clone())
	} else {
		None
	}
}