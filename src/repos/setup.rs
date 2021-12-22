use std::{path::Path, io::ErrorKind};

use super::{
    config::Config, 
    errors::SetupError, 
    utils::PackageFormat
};

#[allow(deprecated)]
pub fn setup() -> Result<Config, SetupError> {
    let home = std::env::home_dir().unwrap()
    .into_os_string().into_string().unwrap();
    let root = format!("{}/.opm/", home);

    let config;
    Config::from(&format!("{}/.config.json", root));

    if let Some(fmt) = PackageFormat::get_format() {
        match fmt {
            PackageFormat::Deb => config = Config::new("deb").unwrap(),
            PackageFormat::Rpm => panic!("We do not support RPM packages for now ..."),
            PackageFormat::Other => panic!("Unrecognized package"),
        }
    } else {
        eprintln!("Consider define `PKG_FMT` environment variable!");
        std::process::exit(1);
    }
    
    if !Path::new(&config.root).exists() {
        config.setup()?;
    }

    Ok(config)
}

#[allow(deprecated)]
pub fn roll_back() {
    println!("Rolling back ...");
    let home = std::env::home_dir().unwrap()
    .into_os_string().into_string().unwrap();
    let root = format!("{}/.opm/", home);

    match std::fs::remove_dir_all(root){
        Ok(_) => (),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => (),
            _ => panic!("fuck {}", e)
        }
    }
}