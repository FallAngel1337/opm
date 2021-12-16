use std::{path::PathBuf, cell::RefCell};
use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct SQLite {
    pub db: PathBuf,
    conn: RefCell<Option<Connection>>
}

impl SQLite {
    pub fn new(db: &PathBuf) -> Result<Self> {
        let mut sql = Self {
            db: db.to_path_buf(),
            conn: RefCell::new(Some(Connection::open(&db)?))
        };
        sql.init()?;
        Ok(sql)
    }

    pub fn execute<P: rusqlite::Params>(&self, query: &str, params: P) -> Result<()> {
        if let Some(sqlite) = self.conn.borrow().as_ref() {
            sqlite.execute(query, params)?;
        }
        Ok(())
    }

    pub fn get_conn(&self) -> std::cell::Ref<Option<Connection>> {
        self.conn.borrow()
    }

    pub fn close(&self) {
        *(self.conn.borrow_mut()) = None;
    }

    fn init(&mut self) -> Result<()> {
        self.execute(
            "CREATE TABLE IF NOT EXISTS deb_cache (
                package text not null primary key,
                version text not null,
                architecture text not null,
                maintainer text not null,
                description text not null,
                filename text not null,
                size integer not null,
                md5sum text not null,
                sha1 text not null,
                sha256 text not null,
                sha512 text not null
            )", []
        )?;
        
        self.execute(
            "CREATE TABLE IF NOT EXISTS deb_installed (
                package text not null primary key,
                version text not null,
                architecture text not null,
                maintainer text not null,
                description text not null,
                filename text not null,
                size integer not null,
                md5 text not null,
                sha1 text not null,
                sha256 text not null,
                sha512 text not null
            )", []
        )?;
        
        // add more fields/tables in the future

        Ok(())
    }
}