use super::deb::package::DebPackage;
use rusqlite::{Connection, Result};
use sha2::{Sha256, Digest};

#[derive(Debug)]
pub enum Packages {
    DebPackage(DebPackage),
}

#[derive(Debug)]
pub struct SQLite<'a> {
    pub db: &'a str,
    conn: Option<Connection>
}

impl<'a> SQLite<'a> {
    pub fn new(db: &'a str) -> Self {
        SQLite {
            db,
            conn: None
        }
    }

    pub fn init(&mut self) -> Result<()> {
        match self.conn {
            Some(_) => (),
            None => self.conn = Some(Connection::open(self.db)?)
        }

        Ok(())
    }

    pub fn create_tables(&self) -> Result<()> {
        self.conn.as_ref().unwrap().execute(
            "create table if not exists deb_pkgs (
                id string primary key,
                name text not null,
                version text not null,
                dependencies text not null
            );
            create table if not exists debsrc_pkgs (
                id integer primary key,
                name text not null
            )",
            // add more fields in the future
            []
        )?;

        Ok(())
    }

    pub fn register(&self, pkg: Packages) -> Result<()> {
        let mut hasher = Sha256::new();

        match pkg {
            Packages::DebPackage(pkg) => {
                let nodep = String::from("NOPE");
                let pkg_name = pkg.control.fields.get("Package").unwrap();
                let pkg_version = pkg.control.fields.get("Version").unwrap();
                let pkg_dependencies = pkg.control.fields.get("Dependencies").unwrap_or(&nodep); /* Turn this in to a Vec<Dependencies> */

                hasher.update(format!("{}{}", pkg_name, pkg_version)); // Not sure if it's this way
                let result = hasher.finalize();

                self.conn.as_ref().unwrap().execute(
                    "insert into deb_pkgs (id, name, version, dependencies)
                    values (?, ?, ?, ?)",
                    [&hex::encode(result), pkg_name, pkg_version, pkg_dependencies]
                )?;
            }
        };

        Ok(())
    }
}