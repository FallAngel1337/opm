use ar::Archive;
use tar::Archive as tarar;
use xz2::read::XzDecoder;

use std::fs::{self, File};
use std::io::{self, prelude::*};
use std::str;
use std::path::Path;

use crate::repos::errors::InstallError;
use super::package::{DebPackage, PkgKind};

pub fn extract(package: &str, to: &Path) -> Result<DebPackage, InstallError> {
    let mut archive = Archive::new(File::open(package)?);
    
    let mut bytes: Vec<u8> = Vec::new();
    let mut file = File::open(package)
        .unwrap_or_else(|_| panic!("Could not open the file `{}`", package));
    file.read_to_end(&mut bytes)
        .unwrap_or_else(|_| panic!("Could not read the file `{}`", package));

    while let Some(entry_result) = archive.next_entry() {
        let mut entry = entry_result?;
        
        let filename = str::from_utf8(entry.header().identifier()).unwrap().to_string();
        let mut file = File::create(&filename)
            .expect("Could not create package file");

        io::copy(&mut entry, &mut file)
            .expect("Could not copy the contents of the file");
        
        if filename.contains(".tar.xz") {
            let file = File::open(&filename)?;

            let tar = XzDecoder::new(file);
            let mut archive = tarar::new(tar);

            archive.unpack(to)?;
        }

        fs::remove_file(&filename)
            .unwrap_or_else(|_| panic!("Could not remove `{}`", filename));
    }

    Ok(
        DebPackage::new(&format!("{}/control", to.to_path_buf().into_os_string()
                                                .into_string().unwrap()), PkgKind::Binary)?
    )
}
