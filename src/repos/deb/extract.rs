use ar::Archive;
use tar::Archive as tarar;
use xz2::read::XzDecoder;
use flate2::read::GzDecoder;

use std::fs::{self, File};
use std::io::{self, prelude::*};
use std::str;

use crate::repos::{errors::InstallError, config::Config};
use super::package::{DebPackage, PkgKind};

fn unpack(filename: &str, dst: &str) -> std::io::Result<()> {
    let file = File::open(&filename)?;

    if filename.ends_with(".tar.gz") {
        let tar = GzDecoder::new(file);
        let mut archive = tarar::new(tar);
        archive.unpack(dst)?;
    } else if filename.ends_with(".tar.xz") {
        let tar = XzDecoder::new(file);
        let mut archive = tarar::new(tar);
        archive.unpack(dst)?;
    }

    Ok(())
}

pub fn extract(config: &Config, path: &str, pkg: &str) -> Result<DebPackage, InstallError> {
    let mut archive = Archive::new(File::open(path).expect("msg"));

    let mut bytes: Vec<u8> = Vec::new();
    let mut file = File::open(path)
        .unwrap_or_else(|_| panic!("Could not open the file `{}`", path));
    file.read_to_end(&mut bytes)
        .unwrap_or_else(|_| panic!("Could not read the file `{}`", path));

    let package_dst = format!("{}/{}", config.tmp.clone().into_os_string().into_string().unwrap(), pkg);
    let control_dst = format!("{}/{}", config.info.clone().into_os_string().into_string().unwrap(), pkg);

    std::fs::create_dir_all(&package_dst)?;
    std::fs::create_dir_all(&control_dst)?;

    while let Some(entry_result) = archive.next_entry() {
        let mut entry = entry_result?;
        
        let filename = str::from_utf8(entry.header().identifier()).unwrap().to_string();
        let mut file = File::create(&filename)
            .expect("Could not create path file");

        io::copy(&mut entry, &mut file)
            .expect("Could not copy the contents of the file");

        match filename.as_ref() {
            "data.tar.xz"|"data.tar.gz" => unpack(&filename, &package_dst).expect("msg"),
            "control.tar.xz"|"control.tar.gz" => unpack(&filename, &control_dst).expect("msg"),
            _ => ()
        }

        fs::remove_file(&filename)
            .unwrap_or_else(|_| panic!("Could not remove `{}`", filename));
    }

    let control_file = &format!("{}/control", control_dst);

    Ok(
        DebPackage::new(control_file, PkgKind::Binary).expect("msg")
    )
}
