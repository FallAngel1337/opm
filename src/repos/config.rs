use std::path::PathBuf;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub cache: PathBuf,
    pub pkgs: PathBuf,
    pub tmp: PathBuf
}

impl Config {
    pub fn new() -> Self {
        let home = env::home_dir().unwrap()
            .into_os_string().into_string().unwrap();
        Config {
            cache: PathBuf::from(format!("{}/.rpm/cache", home)),
            pkgs: PathBuf::from(format!("{}/.rpm/pkgs", home)),
            tmp: PathBuf::from(format!("{}/.rpm/tmp", home))
        }
    }
}