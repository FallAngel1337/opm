use anyhow::Result;
use std::io::{self, ErrorKind, Write};
use crate::repos::errors::ConfigError;

use super::{config::Config, os_fingerprint::OsInfo};

fn get_answer() -> Result<String> {
    let mut answer = String::new();
    io::stdout().flush()?;
    io::stdin().read_line(&mut answer)?;
    Ok(answer)
}

pub fn setup() -> Result<Config> {
    let os_info = OsInfo::new()?;
    let config_file = os_info.install_dir.join("config.json");

    if config_file.exists() {
        Ok(Config::from(config_file))
    } else {
        let curr_conf = Config::new(&os_info)?;
        
        if !os_info.install_dir.exists() {
            curr_conf.setup()?;
        }

        println!("Got default configuration:\n{:#?}", curr_conf);
        print!("Want to keep those? [y/n] ");

        if get_answer()?.to_ascii_lowercase().trim().starts_with('y') {
            curr_conf.save(&config_file);
            Ok(curr_conf)
        } else {
            curr_conf.save(&config_file);
            anyhow::bail!(ConfigError::ChangeConfig(config_file.to_str().unwrap().to_owned()))
        }
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