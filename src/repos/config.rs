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
            
        let mut config = Self {
            cache: PathBuf::from(format!("{}/.opm/cache/pkg_cache", home)),
            db: PathBuf::from(format!("{}/.opm/db/", home)),
            rls: PathBuf::from(format!("{}/.opm/cache/rls", home)),
            tmp: PathBuf::from(format!("{}/.opm/tmp", home)),
            sqlite: None
        };
        
        Self::setup(&mut config)?;
        config.sqlite = Some(SQLite::new(&config.db).unwrap());

        Ok(config)
    }
    
    fn setup(&mut self) -> Result<(), Error> {
        match fs::create_dir_all(&self.db) {
            Ok(_) => (),
            Err(e) => match e.kind() {
                ErrorKind::AlreadyExists => (),
                _ => panic!("Some error occurred {}", e)
            }
        }

        self.db.push("pkgs.db");

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