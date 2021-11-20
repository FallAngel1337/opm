use ar::Archive;
use tar::Archive as tarar;
use xz::read::XzDecoder;
use sha2::{Sha256, Digest};

use std::fs::{self, File};
use std::io::{self, prelude::*};
use std::str;
use std::path::PathBuf;

use crate::repos::errors::InstallError;
use super::package::{DebPackage, PkgKind};

pub fn extract(package: &str, to: &PathBuf) -> Result<DebPackage, InstallError> {
    let mut archive = Archive::new(File::open(package)?);
    
    let mut bytes: Vec<u8> = Vec::new();
    let mut file = File::open(package)?;
    let mut hasher = Sha256::new();
    file.read_to_end(&mut bytes)?;

    hasher.update(bytes);
    let sig = hasher.finalize();

    while let Some(entry_result) = archive.next_entry() {
        let mut entry = entry_result?;
        
        let filename = str::from_utf8(entry.header().identifier()).unwrap().to_string();
        let mut file = File::create(&filename)?;

        io::copy(&mut entry, &mut file)?;
        
        match filename.find("data.tar.xz") {
            Some(_) => {
                let file = File::open(&filename)?;

                let tar = XzDecoder::new(file);
                let mut archive = tarar::new(tar);

                archive.unpack(to)?;
            }
            None => ()
        }

        match filename.find("control.tar.xz") {
            Some(_) => {
                let file = File::open(&filename)?;

                let tar = XzDecoder::new(file);
                let mut archive = tarar::new(tar);

                archive.unpack(to)?;
            }
            None => ()
        }

        fs::remove_file(&filename)?;
    }

    Ok(
        DebPackage::new(&format!("{}/control", to.clone().into_os_string().into_string().unwrap()), PkgKind::Binary, hex::encode(sig))?
    )
}
