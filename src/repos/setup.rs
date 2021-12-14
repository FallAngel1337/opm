use std::path::Path;

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
                config.setup_db()?;
                cache::populate_db(config)?;
            }
            _ => ()
        }
    } else {
        config.setup()?;
        config.setup_db()?;
        cache::populate_db(config)?;
    }

    let is_empty = config.cache.read_dir()?.next().is_none();

    if is_empty {
        println!("Consider do `opm update` before continue ...");
        return Ok(());
    } else {
        println!("Updating the cache ...");
        cache::update_cache(&config)?;
    }

    Ok(())
}