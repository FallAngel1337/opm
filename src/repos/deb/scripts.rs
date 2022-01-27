use anyhow::{self, Result};
use crate::repos::errors::ScriptsError;
use std::process::Command;

use super::package::Info;

///
/// Pre/Post install/remove scripts execution
///
pub fn execute_install_pre(i: &Info) -> Result<()>{
    if let Some(preinst) = &i.preinst {
        print!("Running pre-install script ...");
        let path = preinst.clone().into_os_string().into_string().unwrap();
        match Command::new("sh").args(["-c", &path]).output() {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Failed to install the package due {}\nRemoving it ...", e);
                execute_remove_pre(i)?;
                execute_remove_pos(i)?;
                anyhow::bail!(ScriptsError::PreInstError);
            }
        }
        println!("Done");
    }
    
    Ok(())
}

pub fn execute_install_pos(i: &Info) -> Result<()>{
    if let Some(postinst) = &i.postinst {
        print!("Running post-install script ...");
        let path = postinst.clone().into_os_string().into_string().unwrap();
        match Command::new("sh").args(["-c", &path]).output() {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Failed to install the package due {}\nRemoving it ...", e);
                execute_remove_pre(i)?;
                execute_remove_pos(i)?;
                anyhow::bail!(ScriptsError::PostInstError);
            }
        }
        println!("Done");
    }
    
    Ok(())
}

pub fn execute_remove_pre(i: &Info) -> Result<()> {
    if let Some(prerm) = &i.prerm {
        print!("Running pre-remove script ...");
        let path = prerm.clone().into_os_string().into_string().unwrap();
        match Command::new("sh").args(["-c", &path]).output() {
            Ok(_) => (),
            Err(e) => {
                println!();
                eprintln!("Failed to remove the package due {}\nRemoving it ...", e);
                anyhow::bail!(ScriptsError::PreRmError);
            }
        }
        println!("Done");
    }

    Ok(())
}

pub fn execute_remove_pos(i: &Info) -> Result<()> {  
    if let Some(postrm) = &i.postrm {
        print!("Running post-remove script ...");
        let path = postrm.clone().into_os_string().into_string().unwrap();
        match Command::new("sh").args(["-c", &path]).output() {
            Ok(_) => (),
            Err(e) => {
                println!();
                eprintln!("Failed to remove the package due {}\nRemoving it ...", e);
                anyhow::bail!(ScriptsError::PostRmError);
            }
        }
        println!("Done");
    }

    Ok(())
}

