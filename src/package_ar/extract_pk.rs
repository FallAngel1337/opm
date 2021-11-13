use ar::Archive;
use tar::Archive as tarar;
use xz::read::XzDecoder;

use std::fs::{self, File};
use std::io;
use std::str;

pub fn extract(package: &str) -> io::Result<()> {
    let mut archive = Archive::new(File::open(package)?);

    while let Some(entry_result) = archive.next_entry() {
        let mut entry = entry_result?;
        
        let filename = str::from_utf8(entry.header().identifier()).unwrap().to_string();
        let mut file = File::create(&filename)?;

        io::copy(&mut entry, &mut file)?;
        
        match filename.find("tar.xz") {
            Some(_) => {
                let file = File::open(&filename)?;

                let tar = XzDecoder::new(file);
                let mut archive = tarar::new(tar);

                archive.unpack(super::OUTDIR)?;
            }
            None => ()
        }

        fs::remove_file(&filename)?;
    }

    Ok(())
}
