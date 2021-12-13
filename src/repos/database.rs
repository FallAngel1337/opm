use std::path::{Path, PathBuf};
use rusqlite::{Connection, Result, params};
use super::{utils::PackageFormat, config::Config};
use super::cache;

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
        if !db.ends_with(".db") {
            db.push("installed.db");
            if Path::new(db).exists() {
                let mut sql = Self {
                    db: db.to_path_buf(),
                    conn: Connection::open(db)?
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
        } else {
            let mut sql = Self {
                db: db.to_path_buf(),
                conn: Connection::open(db)?
            };
            Ok(sql)
        }
    }

    fn init(&mut self, config: &mut Config) -> Result<()> {
        self.conn.execute(
            "create table if not exists deb_pkgs (
                id string not null,
                name text not null primary key,
                version text not null
            );
            create table if not exists debsrc_pkgs (
                id integer primary key,
                name text not null
            )",
            // add more fields in the future
            []
        )?;

        cache::dump_into_db(config);

        Ok(())
    }

    pub fn add_package<P: Package>(&self, package: P) -> Result<()> {
        let package = package.to_generic();

        let table = match package.format {
            PackageFormat::Deb => "deb_pkgs",
            _ => panic!("Invalid format in the db")
        };

        self.conn.execute(
            &format!("insert into {}(id, name, version) values (?1, ?2, ?3)", table),
            params![package.id, package.name, package.version],
        )?;

        Ok(())
    }
}