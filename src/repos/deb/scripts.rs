use anyhow::{self, Result};
use crate::repos::errors::InstallError;
use std::path::Path;
use std::process::Command;

///
/// Pre/Post install/remove scripts execution
/// TODO: Make this better to execute
pub fn execute_install(p: &Path) -> Result<()>{
    let p = Path::new(p);
    
    if !p.is_dir() {
        anyhow::bail!(InstallError::Error("Package extration folder is not a directory".to_owned()));
    } else {
        for entry in std::fs::read_dir(p).unwrap() {

            let path = entry
                .as_ref()
                .unwrap()
                .path()
                .into_os_string()
                .into_string()
                .unwrap();

            let script = &path
                .rsplit('/')
                .next()
                .unwrap()
                .to_owned();

            match script.as_ref() {
                "preinst" => {
                    if Command::new("sh").args(["-c", &path]).output().is_err() {
                        eprintln!("Failed to install the package!\nRemoving it ...");
                        execute_remove(p)?;
                        anyhow::bail!(InstallError::Error("Could not execute preinst the script".to_owned()))
                    }
                },

                "postinst" => {
                    if Command::new("sh").args(["-c", &path]).output().is_err() {
                        eprintln!("Failed to configure the package!");
                        anyhow::bail!(InstallError::Error("Could not execute postinst the script".to_owned()))
                    }
                },

                _ => ()
            }
        }
        
        Ok(())
    }
}

// TODO: Add a `purge` option
pub fn execute_remove(p: &Path) -> Result<(), InstallError> {
    if !p.is_dir() {
        Err(InstallError::Error("Package extration folder is not a directory".to_owned()))
    } else {
        for entry in std::fs::read_dir(p).unwrap() {
            let path = entry
                .as_ref()
                .unwrap()
                .path()
                .into_os_string()
                .into_string()
                .unwrap();

            let script = &path
                .rsplit('/')
                .next()
                .unwrap()
                .to_owned();

            match script.as_ref() {
                "prerm" => {
                    if Command::new("sh").args(["-c", &path]).output().is_err() {
                        eprintln!("Failed to install the package!\nRemoving it ...");
                        execute_remove(p)?;
                        return Err(InstallError::Error("Could not execute preinst the script".to_owned()))
                    }
                },

                "postrm" => {
                    if Command::new("sh").args(["-c", &path]).output().is_err() {
                        eprintln!("Failed to install the package!\nRemoving it ...");
                        execute_remove(p)?;
                        return Err(InstallError::Error("Could not execute postinst the script".to_owned()))
                    }
                },

                _ => ()
            }
        }
        
        Ok(())
    }
}

