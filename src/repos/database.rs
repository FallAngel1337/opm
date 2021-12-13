use std::path::{Path, PathBuf};
use rusqlite::{Connection, Result};
use super::{utils::PackageFormat, config::Config};
use super::cache;

#[derive(Debug)]
pub struct GenericPackage {
    pub id: String,
    pub name: String,
    pub version: String,
    pub format: PackageFormat
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
    pub fn new(db: &mut PathBuf, config: &mut Config) -> Result<Self> {
        db.push("installed.db");
        if Path::new(db).exists() {
            let sql = Self {
                db: db.to_path_buf(),
                conn: Connection::open(&db)?
            };
            Ok(sql)
        } else {
            let mut sql = Self {
                db: db.to_path_buf(),
                conn: Connection::open(db)?
            };
            sql.init(config)?;
            Ok(sql)
        }
    }

    fn init(&mut self, config: &mut Config) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS deb_pkgs (
                id string not null,
                name text not null primary key,
                version text not null
            );
            CREATE TABLE IF NOT EXISTS debsrc_pkgs (
                id integer primary key,
                name text not null
            )",
            // add more fields in the future
            []
        )?;

        cache::dump_into_db(config)?;

        Ok(())
    }

    pub fn add_package<P: Package>(&self, package: P) -> Result<()> {
        let package = package.to_generic();

        let table = match package.format {
            PackageFormat::Deb => "deb_pkgs",
            _ => panic!("Invalid format in the db")
        };

        self.conn.execute(
            &format!("INSERT INTO {}(id, name, version) VALUES (?1, ?2, ?3)", table),
            [package.id, package.name, package.version],
        )?;

        Ok(())
    }

    // TODO: Return a trait object and remove hardcoded table
    pub fn lookup(&self, name: &str) -> Result<()> {
        let mut result = self.conn.prepare(
            "SELECT * FROM deb_pkgs WHERE name = ?1",
        )?;

        let package = result.query_map([name], |row| {
            Ok (
                GenericPackage {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    version: row.get(2)?,
                    format: PackageFormat::Deb
                }
            )
        })?;

        for pkg in package {
            println!("PKG>> {:?}", pkg);
        };

        Ok(())
    }
}