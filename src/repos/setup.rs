use anyhow::Result;
use std::{path::Path, io::{self, ErrorKind, Write}};
use super::{config::Config, utils::PackageFormat};

fn get_answer() -> Result<String> {
    let mut answer = String::new();
    io::stdout().flush()?;
    io::stdin().read_line(&mut answer)?;
    Ok(answer)
}

pub fn setup(pkg_fmt: Option<&str>) -> Result<Config> {
    #[allow(deprecated)]
    let home = std::env::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
        
    let opm_dir = format!("{}/.opm/", home);
    let config_file = format!("{}/config.toml", opm_dir);

    if let Some(pkg_fmt) = pkg_fmt {
        Ok(Config::new(pkg_fmt)?)
    } else if !Path::new(&config_file).exists() {
        println!("Entering setup mode ...");
        let config = match PackageFormat::get_format()? {
            PackageFormat::Deb => {
                print!("Are you on a Debian-based distro? [y/n] ");
                if get_answer()?.to_ascii_lowercase().trim().starts_with('y') {
                    Config::new("deb")?
                } else {
                    print!("Insert the package format: ");
                    Config::new(get_answer()?.trim())?
                }
            },
            PackageFormat::Rpm => {
                print!("Are you on a RHEL-based distro? [y/n] ");
                if get_answer()?.to_ascii_lowercase().trim().starts_with('y') {
                    Config::new("rpm")?
                } else {
                    print!("Insert the package format: ");
                    Config::new(get_answer()?.trim())?
                }
            }
            PackageFormat::Other => panic!("Unrecognized package"),
        };
        
        if !Path::new(&opm_dir).exists() {
            config.setup()?;
        }

        println!("Done");
        let config_file = format!("{}config.toml", opm_dir);
        println!("Saving config file to {}", config_file);
        config.save(&config_file);

        Ok(config)
    } else {
        Ok(Config::from(&config_file))
    }
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
            _ => panic!("Clould not rollback due {}", e)
        }
    }
}