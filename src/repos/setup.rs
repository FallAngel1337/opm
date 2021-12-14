use super::config::Config;
use super::cache;
use rusqlite::Result;

pub fn setup(config: &mut Config) -> Result<()> {
    println!("Updating the database ...");

    println!("Dumping installed packages into the database");
    cache::dump_into_db(config)?;

    Ok(())
}