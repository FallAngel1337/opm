use crate::repos::database::SQLite;

use super::{config::Config, errors::SetupError};
use super::cache;

pub fn setup(config: &mut Config) -> Result<(), SetupError> {
    println!("Updating the database ...");
    println!("Removing old {:?}", config.db);
    std::fs::remove_file(&config.db)?;

    config.sqlite = Some(SQLite::new(&config.db)?);
    cache::dump_into_db(config)?;

    Ok(())
}