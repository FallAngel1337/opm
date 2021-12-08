use super::package::ControlFile;
use std::fs;

///
/// Lookup into the dpkg cache (/var/lib/dpkg/status).
/// Will be used to check for dependencies.
/// Note: WAY more slow
/// 
// TODO: Improve it to be less slow
pub fn dpkg_cache_lookup(name: &str) -> Option<ControlFile> {
	let control = fs::read_to_string("/var/lib/dpkg/status")
		.unwrap()
		.split("\n\n")
		.map(|ctrl| ControlFile::from(ctrl).unwrap())
		.filter(|ctrl| ctrl.package == name)
		.collect::<Vec<_>>();

	if control.len() > 0 { 
		// println!("Found {} package entries for {}", control.len(), name);
		Some(control[0].clone())
	} else {
		None
	}
}