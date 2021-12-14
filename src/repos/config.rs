use std::path::PathBuf;
use std::io::{Error, ErrorKind};
use std::env;
use std::fs;

use super::database::SQLite;

#[derive(Debug)]
pub struct Config {
    pub cache: PathBuf,
    pub db: PathBuf,
    pub rls: PathBuf,
    pub tmp: PathBuf,

    pub sqlite: Option<SQLite>,
}

#[allow(deprecated)]
impl Config {
    pub fn new() -> Result<Self, Error> {
        let home = env::home_dir().unwrap()
            .into_os_string().into_string().unwrap();
         
        Ok(
            Self {
                cache: PathBuf::from(format!("{}/.opm/cache/pkg_cache", home)),
                db: PathBuf::from(format!("{}/.opm/db/pkgs.db", home)),
                rls: PathBuf::from(format!("{}/.opm/cache/rls", home)),
                tmp: PathBuf::from(format!("{}/.opm/tmp", home)),
                sqlite: None
            }
        )
    }
    
    pub fn setup_db(&mut self) -> rusqlite::Result<()> {
        if self.sqlite.as_ref().is_none() {
            self.sqlite = Some(
                SQLite::new(&self.db)?
            );
        }
        Ok(())
    }

    pub fn setup(&mut self) -> Result<(), Error> {
        let path = std::path::Path::new(&self.db);
        let prefix = path.parent().unwrap();
        fs::create_dir_all(prefix)?;

        match fs::create_dir_all(&self.cache) {
            Ok(_) => (),
            Err(e) => match e.kind() {
                ErrorKind::AlreadyExists => (),
                _ => panic!("Some error occurred {}", e)
            }
        }

        match fs::create_dir_all(&self.rls) {
            Ok(_) => (),
            Err(e) => match e.kind() {
                ErrorKind::AlreadyExists => (),
                _ => panic!("Some error occurred {}", e)
            }
        }
        
        match fs::create_dir_all(&self.tmp) {
            Ok(_) => (),
            Err(e) => match e.kind() {
                ErrorKind::AlreadyExists => (),
                _ => panic!("Some error occurred {}", e)
            }
        }


        Ok(())
    }
}