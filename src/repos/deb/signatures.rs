use anyhow::Result;
use sha2::{Sha256, Sha512, Digest};
use sha1::Sha1;
use std::{path::Path, fs};
use super::package::DebPackage;

// TODO: Made this code less... worst
pub fn verify_sig(pkg: &DebPackage, path: &Path) -> Result<bool> {
    let pkg_md5 = &pkg.control.md5sum;
    let pkg_sha1 = &pkg.control.sha1;
    let pkg_sha256 = &pkg.control.sha256;
    let pkg_sha512 = &pkg.control.sha512;

    let data = fs::read(path)?;
    let mut sha1 = Sha1::new();
    let mut sha256 = Sha256::new();
    let mut sha512 = Sha512::new();
    
    sha1.update(&data);
    sha256.update(&data);
    sha512.update(&data);

    let md5_sig = format!("{:x}", md5::compute(&data));

    let mut sha1_sig = String::new();
    sha1.finalize().as_slice()
    .iter()
    .for_each(|c| sha1_sig.push_str(&*format!("{:02x}", c)));

    let mut sha256_sig = String::new();
    sha256.finalize().as_slice()
    .iter()
    .for_each(|c| sha256_sig.push_str(&*format!("{:02x}", c)));

    let mut sha512_sig = String::new();
    sha512.finalize().as_slice()
    .iter()
    .for_each(|c| sha512_sig.push_str(&*format!("{:02x}", c)));

    Ok (
        *pkg_md5 == md5_sig &&
        *pkg_sha1 == sha1_sig &&
        *pkg_sha256 == sha256_sig &&
        *pkg_sha512 == sha512_sig
    )

}