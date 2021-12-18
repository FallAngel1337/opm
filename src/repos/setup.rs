use std::{path::Path, io::ErrorKind};

use super::{config::Config, errors::SetupError};
use super::cache;

pub fn setup(config: &mut Config) -> Result<(), SetupError> {
    println!("Syncing the database ...");
    
    if Path::new(&config.db).exists() {
        println!("It seems you have an old database at {:?}", config.db);
        println!("Do you want to override it? [y/N]");
        
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Could not read from stdin");
        let input = input.trim().to_ascii_lowercase();
        
        match input.as_ref() {
            "y" => {
                println!("Removing old {:?}", config.db);
                std::fs::remove_file(&config.db)?;
                println!("Populating the database");
            }
            _ => ()
        }
    } else {
        config.setup()?;
    }

    let is_empty = config.cache.read_dir()?.next().is_none();

    if is_empty {
        println!("Consider do `opm update` before continue ...");
    }

    Ok(())
}

pub fn roll_back(config: &Config) {
    println!("Rolling back ...");
    match std::fs::remove_dir_all(&config.root){
        Ok(_) => (),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => (),
            _ => panic!("fuck {}", e)
        }
    }
}