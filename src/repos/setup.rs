use anyhow::Result;
use std::io::ErrorKind;

use super::{config::Config, os_fingerprint::OsInfo};

pub fn setup() -> Result<Config> {
    let os_info = OsInfo::new()?;
    let config_file = os_info.install_dir.join("config.json");

    if config_file.exists() {
        Ok(Config::from(config_file))
    } else {
        let curr_conf = Config::new(&os_info)?;
        
        println!("The following config file can be changed later at {:?}\n{:#?}", config_file, curr_conf);
        
        if !os_info.install_dir.exists() {
            curr_conf.setup()?;
            curr_conf.save(&config_file);
        }

        Ok(curr_conf)
    }
}

pub fn roll_back() {
    println!("Rolling back ...");

    match std::fs::remove_dir_all(OsInfo::new().unwrap().install_dir){
        Ok(_) => (),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => (),
            _ => panic!("Clould not rollback due {}", e)
        }
    }
}