use crate::repos::{config::Config, errors::ConfigError};
use super::{
	package::{ControlFile, DebPackage, PkgKind}
};
use std::fs;

///
/// Dump from the downloded files
/// 
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

		control.into_iter().filter_map(|pkg| pkg.ok()).for_each(|mut pkg| {
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
pub fn dump_installed(config: &Config) -> Vec<Result<ControlFile, ConfigError>> {
	let control = fs::read_to_string("/var/lib/dpkg/status").unwrap();

	let control = control
		.split("\n\n")
		.map(|ctrl| ControlFile::from(config, ctrl))
		// .filter_map(|ctrl| ControlFile::from(config, ctrl).ok())
		.collect::<Vec<_>>();
	
	control
}

// TODO: Return a trait object and remove hardcoded table
pub fn db_lookup(config: &mut Config, name: &str, exact_match: bool, cache: bool) -> rusqlite::Result<Vec<DebPackage>> {
	config.setup_db().expect("Failed to setup database");

	let database = if cache {
		"deb_cache"
	} else {
		"deb_installed"
	};

	// Maybe this can introduce some bugs
	let query = if exact_match {
		format!(r#"SELECT * FROM {} WHERE package = '{}'"#, database, name)
	} else {
		// because
		// format!(r#"SELECT * FROM {} WHERE name LIKE '%?1%'"#, database)
		// simply just don't work... and give me `Wrong number of parameters passed to query. Got 1, needed 0`
		format!(r#"SELECT * FROM {} WHERE package LIKE '%{}%'"#, database, name)
	};

	if let Some(sqlite) = config.sqlite.as_ref() {
		let conn = sqlite.get_conn();
		let mut result = conn.as_ref().unwrap().prepare(&query)?;
		let packages = result.query_map([], |row| {
			Ok (
				DebPackage {
					control: ControlFile {
						package: row.get(0)?,
						version: row.get(1)?,
						architecture: row.get(2)?,
						maintainer: row.get(3)?,
						description: row.get(4)?,
						filename: row.get(5)?,
						size: row.get(6)?,
						md5sum: row.get(7)?,
						sha1: row.get(8)?,
						sha256: row.get(9)?,
						sha512: row.get(10)?,
						depends: None
					},
					kind: PkgKind::Binary
				}
			)
		})?;

		Ok(
			packages.into_iter()
				// .filter_map(|pkg| pkg.ok())
				.map(|pkg| pkg.unwrap())
				.collect()
		)
	} else {
		Ok (
			vec![]
		)
	}
}

pub fn pkg_list(config: &Config) -> rusqlite::Result<Vec<DebPackage>> {
	if let Some(sqlite) = config.sqlite.as_ref() {
		let conn = sqlite.get_conn();
		let mut result = conn.as_ref().unwrap().prepare("SELECT * FROM deb_installed")?;
		let package = result.query_map([], |row| {
			Ok (
				DebPackage {
					control: ControlFile {
						package: row.get(0)?,
						version: row.get(1)?,
						architecture: row.get(2)?,
						maintainer: row.get(3)?,
						description: row.get(4)?,
						filename: row.get(5)?,
						size: row.get(6)?,
						md5sum: row.get(7)?,
						sha1: row.get(8)?,
						sha256: row.get(9)?,
						sha512: row.get(10)?,
						depends: None
					},
					kind: PkgKind::Binary
				}
			)
		})?;

		Ok(
			package.map(|pkg| pkg.unwrap()).collect::<Vec<_>>()
		)
	} else {
		Ok (
			vec![]
		)
	}
}

pub fn add_package(config: &Config, pkg: DebPackage, cache: bool) -> rusqlite::Result<()> {
	 let table = if cache {
		"deb_cache"
	} else {
		"deb_installed"
	};
	if let Some(sqlite) = config.sqlite.as_ref() {
		let query = format!(r#"INSERT INTO {} VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)"#, table);
		let control = &pkg.control;

		sqlite.execute(&query, [
			&control.package,
			&control.version,
			&control.architecture,
			&control.maintainer,
			&control.description,
			&control.filename,
			&control.size,
			&control.md5sum,
			&control.sha1,
			&control.sha256,
			&control.sha512])?;
	}

	Ok(())
}