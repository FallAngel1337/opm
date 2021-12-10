use std::path::PathBuf;
use std::io::{Error, ErrorKind};
use std::env;
use std::fs;

#[derive(Debug, Clone)]
pub struct Config {
    pub cache: PathBuf,
    pub pkgs: PathBuf,
    pub rls: PathBuf,
    pub tmp: PathBuf,
}

#[allow(deprecated)]
impl Config {
    pub fn new() -> Self {
        let home = env::home_dir().unwrap()
            .into_os_string().into_string().unwrap();
        Config {
            cache: PathBuf::from(format!("{}/.opm/cache/pkg_cache", home)),
            pkgs: PathBuf::from(format!("{}/.opm/pkgs", home)),
            rls: PathBuf::from(format!("{}/.opm/cache/rls", home)),
            tmp: PathBuf::from(format!("{}/.opm/tmp", home))
        }
    }

    pub fn setup(&self) -> Result<(), Error> {
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