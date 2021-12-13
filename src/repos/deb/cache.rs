use crate::repos::{config::Config, database::SQLite};
use super::package::ControlFile;
use rusqlite::{Result};
use std::fs;

///
/// Lookup into the dpkg cache (/var/lib/dpkg/status).
/// Will be used to check for dependencies.
/// Note: WAY more slow
/// 
// TODO: Improve it to be less slow
fn dpkg_cache_lookup(config: &Config, name: &str, exact_match: bool) -> Option<Vec<ControlFile>> {
	let control = fs::read_to_string("/var/lib/dpkg/status").unwrap();

	let control = control
		.split("\n\n")
		.map(|ctrl| ControlFile::from(config, ctrl));

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
		Some(control)
	} else {
		None
	}
}

pub fn dpkg_cache_dump(config: &Config) -> Result<Vec<ControlFile>> {
	let control = fs::read_to_string("/var/lib/dpkg/status").unwrap();

	let control = control
		.split("\n\n")
		.map(|ctrl| ControlFile::from(config, ctrl))
		.collect::<Vec<_>>();
	
	Ok(control)
}