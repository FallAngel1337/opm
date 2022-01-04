use anyhow::{self, Result};
use crate::repos::errors::InstallError;
use std::process::Command;

use super::package::Info;

///
/// Pre/Post install/remove scripts execution
/// TODO: Make this better to execute
pub fn execute_install(i: &Info) -> Result<()>{
    if let Some(preinst) = &i.preinst {
        print!("Running pre-install script ...");
        let path = preinst.clone().into_os_string().into_string().unwrap();
        match Command::new("sh").args(["-c", &path]).output() {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Failed to install the package due {}\nRemoving it ...", e);
                execute_remove(i)?;
                anyhow::bail!(InstallError::Error("Could not execute preinst the script".to_owned()))
            }
        }
        println!("Done");
    }
    
    if let Some(postinst) = &i.postinst {
        print!("Running post-install script ...");
        let path = postinst.clone().into_os_string().into_string().unwrap();
        match Command::new("sh").args(["-c", &path]).output() {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Failed to install the package due {}\nRemoving it ...", e);
                execute_remove(i)?;
                anyhow::bail!(InstallError::Error("Could not execute preinst the script".to_owned()))
            }
        }
        println!("Done");
    }
    
    Ok(())
}

// TODO: Add a `purge` option
pub fn execute_remove(i: &Info) -> Result<()> {
    if let Some(prerm) = &i.prerm {
        print!("Running pre-remove script ...");
        let path = prerm.clone().into_os_string().into_string().unwrap();
        match Command::new("sh").args(["-c", &path]).output() {
            Ok(_) => (),
            Err(e) => {
                println!();
                eprintln!("Failed to remove the package due {}\nRemoving it ...", e);
                anyhow::bail!(InstallError::Error("Could not execute preinst the script".to_owned()))
            }
        }
        println!("Done");
    }
    
    if let Some(postrm) = &i.postrm {
        print!("Running post-remove script ...");
        let path = postrm.clone().into_os_string().into_string().unwrap();
        match Command::new("sh").args(["-c", &path]).output() {
            Ok(_) => (),
            Err(e) => {
                println!();
                eprintln!("Failed to remove the package due {}\nRemoving it ...", e);
                anyhow::bail!(InstallError::Error("Could not execute preinst the script".to_owned()))
            }
        }
        println!("Done");
    }

    Ok(())
}

