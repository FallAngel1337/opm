use super::{config::Config, errors::SetupError};
use super::cache;

pub fn setup(config: &mut Config) -> Result<(), SetupError> {
    println!("Updating the database ...");

    cache::dump_into_db(config)?;
    // more setup configurations in the future, if need

    Ok(())
}