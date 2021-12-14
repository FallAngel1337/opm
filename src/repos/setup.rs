use crate::repos::database::SQLite;

use super::{config::Config, errors::SetupError};
use super::cache;

pub fn setup(config: &mut Config) -> Result<(), SetupError> {
    println!("Updating the database ...");
    println!("Removing old {:?}", config.db);
    std::fs::remove_file(&config.db)?;

    config.sqlite = Some(SQLite::new(&config.db)?);
    cache::dump_into_db(config)?;

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