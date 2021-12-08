// use super::config::DebianSource;
use crate::repos::errors::InstallError;
use crate::repos::config::Config;
use crate::repos::cache;
// use deb_version;


#[tokio::main]
pub async fn download(config: &Config, name: &str) -> Result<(), InstallError> {
    println!("Downloading {} from {:?}", name, config.cache);

    match cache::cache_lookup(config, name) {
        Some(v) => {
            v.iter().for_each(|pkg| {
                println!(">> {}", pkg.0.version);
                println!("-> {:?}", pkg.0.depends);
            });
        },

        None => println!("Package {} not found", name)
    };

    Ok(())
}