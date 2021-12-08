use std::path::PathBuf;
// use super::deb::package::DebPackage;
use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct SQLite<'a> {
    pub db: &'a PathBuf,
    conn: Option<Connection>
}

impl<'a> SQLite<'a> {
    pub fn new(db: &'a mut PathBuf) -> Self {
        db.push("installed.db");
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

    // pub fn register(&self, pkg: DebPackage) -> Result<()> {
    //     self.conn.as_ref().unwrap().execute(
    //         "insert into deb_pkgs (id, name, version, dependencies)
    //         values (?, ?, ?, ?)",
    //         [pkg.signature, pkg.control.package, pkg.control.version, pkg.control.depends]
    //     )?;
    //     Ok(())
    // }
}