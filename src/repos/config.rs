use std::path::PathBuf;
use std::io::{Error, ErrorKind};
use std::env;
use std::fs;

use super::database::SQLite;

#[derive(Debug)]
pub struct Config {
    pub cache: PathBuf,
    pub pkgs: PathBuf,
    pub rls: PathBuf,
    pub tmp: PathBuf,

    pub sqlite: Option<SQLite>,
}

#[allow(deprecated)]
impl Config {
    pub fn new() -> Result<Self, Error> {
        let home = env::home_dir().unwrap()
            .into_os_string().into_string().unwrap();
        let mut result = Self {
            cache: PathBuf::from(format!("{}/.opm/cache/pkg_cache", home)),
            pkgs: PathBuf::from(format!("{}/.opm/pkgs", home)),
            rls: PathBuf::from(format!("{}/.opm/cache/rls", home)),
            tmp: PathBuf::from(format!("{}/.opm/tmp", home)),

            sqlite: None
        };

        Self::setup(&mut result)?;
        result.sqlite = Some(SQLite::new(&mut result.pkgs.clone()).unwrap());
        Ok(result)
    }

    fn setup(&mut self) -> Result<(), Error> {
        match fs::create_dir_all(&self.cache) {
            Ok(_) => (),
            Err(e) => match e.kind() {
                ErrorKind::AlreadyExists => (),
                _ => panic!("Some error occurred {}", e)
            }
        }
        
        match fs::create_dir_all(&self.pkgs) {
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