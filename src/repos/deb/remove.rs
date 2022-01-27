use anyhow::Result;
use std::{path::Path, fs};
use super::cache;
use crate::repos::{errors::RemoveError, config::Config, deb::{package::Info, scripts}};

pub fn remove(config: &Config, name: &str, purge: bool) -> Result<()> {
    if cache::check_installed(config, name).is_some() {
        let info = Info::load(Path::new(&format!("{}/{}", config.info, name)))?;

        if let Some(md5sums) = &info.md5sums {
            // TODO: Remove this workaround
            let mut files = fs::read_to_string(md5sums)?.lines()
                .map(|line| line.split(' '))
                .flatten()
                .filter(|f| f.contains('/'))
                .map(|f| format!("/{}", f))
                .collect::<Vec<_>>();
                
            if purge {
                println!("Purging {} ...", name);
                if let Some(conffiles) = &info.conffiles {
                    let mut conffiles = fs::read_to_string(conffiles)?.lines()
                        .map(|line| line.to_owned())
                        .collect::<Vec<_>>();
                    files.append(&mut conffiles);
                }

                scripts::execute_remove_pre(&info)?;
                fs_extra::remove_items(&files)?;
                scripts::execute_install_pos(&info)?;
                cache::rm_package(config, name)?;
            } else {
                println!("Removing {} ...", name);
                if let Some(bin) = files.into_iter().find(|name| name.contains("bin/")) {
                    scripts::execute_remove_pre(&info)?;
                    fs::remove_file(bin)?;
                    scripts::execute_install_pos(&info)?;
                    cache::rm_package(config, name)?;
                }
            }

            Ok(())
        } else {
            anyhow::bail!(RemoveError::NotFoundError(name.to_owned()));
        }
            
    } else {
        anyhow::bail!(RemoveError::NotFoundError(name.to_owned()));
    }
}