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
        // pType      TEXT CHECK( pType IN ('M','R','H') )   NOT NULL DEFAULT 'M',
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS deb_pkgs (
                id text not null,
                name text not null primary key,
                version text not null,
                installed text check (installed in ('Y', 'N')) NOT NULL
            );
            CREATE TABLE IF NOT EXISTS debsrc_pkgs (
                id integer primary key,
                name text not null
            )",
            // add more fields in the future
            []
        )?;

        Ok(())
    }

    pub fn add_package<P: Package>(&self, package: P) -> Result<()> {
        let package = package.to_generic();

        let table = match package.format {
            PackageFormat::Deb => "deb_pkgs",
            _ => panic!("Invalid format in the db")
        };

        let status = match package.status {
            PackageStatus::Installed => "Y",
            PackageStatus::NotInstalled => "N",
            _ => panic!("Invalud status")
        };

        self.conn.execute(
            &format!("INSERT INTO {} VALUES (?1, ?2, ?3, ?4)", table),
            [package.id, package.name, package.version, status.to_owned()],
        )?;

        Ok(())
    }

    // TODO: Return a trait object and remove hardcoded table
    pub fn lookup(&self, name: &str, exact_match: bool) -> Result<Option<Vec<GenericPackage>>> {

        let query = if exact_match {
            "SELECT * FROM deb_pkgs WHERE name = ?1"
        } else {
            "SELECT * FROM deb_pkgs WHERE name LIKE %?1%"
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

        Ok(Some(packages.into_iter().filter_map(|pkg| Some(pkg.unwrap())).collect::<Vec<_>>()))
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