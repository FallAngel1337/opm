use std::path::Path;
use std::process::Command;
use crate::repos::errors::InstallError;

///
/// Pre/Post install/remove scripsts execution
///
pub fn execute(p: &Path) -> Result<(), InstallError>{
    let mut command = Command::new("sh");

    if !p.is_dir() {
        Err(InstallError::Error("Package extration folder is not a directory".to_owned()))
    } else {
        for entry in std::fs::read_dir(p).unwrap() {
            let path = entry
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
                    println!("Executing pre-install script ...");
                    command.args(["-c", &path])
                        .output()
                        .expect("Failed to execute pre-install script");
                },

                "postinst" => {
                    println!("Executing post-install script ...");
                    command.args(["-c", &path])
                        .output()
                        .expect("Failed to execute post-install script");
                },

                _ => ()
            }
        }
        
        Ok(())
    }

}