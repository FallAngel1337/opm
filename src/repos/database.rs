use std::path::PathBuf;
use rusqlite::{Connection, Result};
use super::utils::PackageFormat;

#[derive(Debug)]
pub struct GenericPackage {
    pub id: String,
    pub name: String,
    pub version: String,
    pub format: PackageFormat,
    pub status: PackageStatus
}

#[derive(Debug, Clone)]
pub enum PackageStatus {
    Installed,
    NotInstalled
}

pub trait Package {
    fn to_generic(&self) -> GenericPackage;
}

#[derive(Debug)]
pub struct SQLite {
    pub db: PathBuf,
    conn: Connection
}

impl SQLite {
    pub fn new(db: &PathBuf) -> Result<Self> {
        let mut sql = Self {
            db: db.to_path_buf(),
            conn: Connection::open(&db)?
        };
        sql.init()?;
        Ok(sql)
    }

    fn init(&mut self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS deb_cache (
                id text not null,
                name text not null primary key,
                version text not null
            )", []
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS deb_installed (
                id text not null,
                name text not null primary key,
                version text not null
            )", []
        )?;
        
            // add more fields in the future

        Ok(())
    }

    pub fn add_package<P: Package>(&self, package: P, cache: bool) -> Result<()> {
        let package = package.to_generic();

        let table = match package.format {
            PackageFormat::Deb => if cache {
                "deb_cache"
            } else {
                "deb_installed"
            },
            _ => panic!("Invalid format in the db")
        };

        self.conn.execute(
            &format!("INSERT INTO {} VALUES (?1, ?2, ?3)", table),
            [package.id, package.name, package.version],
        )?;

        Ok(())
    }

    // TODO: Return a trait object and remove hardcoded table
    pub fn lookup(&self, name: &str, exact_match: bool) -> Result<Option<Vec<GenericPackage>>> {

        let query = if exact_match {
            "SELECT * FROM deb_installed WHERE name = ?1"
        } else {
            "SELECT * FROM deb_installed WHERE name LIKE '%?1%'"
        };

        let mut result = self.conn.prepare(
            query
        )?;

        let packages = result.query_map([name], |row| {
            Ok (
                GenericPackage {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    version: row.get(2)?,
                    format: PackageFormat::Deb,
                    status: PackageStatus::Installed
                }
            )
        })?;

        Ok(
            Some (
                packages.into_iter()
                    .filter_map(|pkg| pkg.ok())
                    .collect()
            )
        )
    }

    pub fn pkg_list(&self) -> Result<Vec<GenericPackage>> {
        let mut result = self.conn.prepare(
            "SELECT * FROM deb_pkgs",
        )?;

        let package = result.query_map([], |row| {
            Ok (
                GenericPackage {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    version: row.get(2)?,
                    format: PackageFormat::Deb,
                    status: PackageStatus::Installed
                }
            )
        })?;

        Ok(
            package.map(|pkg| pkg.unwrap()).collect::<Vec<_>>()
        )
    }
}