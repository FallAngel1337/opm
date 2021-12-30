use anyhow::{self, Result};
use crate::repos::errors::SignatureError;
use sha2::{Sha256, Sha512, Digest};
use sha1::Sha1;
use std::{path::Path, fs};
use super::package::DebPackage;

// TODO: Made this code less... worst
pub fn verify_sig(pkg: &DebPackage, path: &Path) -> Result<()> {
    let control = &pkg.control;

    let data = fs::read(path)?;
    
    if !control.md5sum.is_empty() {
        let md5 = format!("{:x}", md5::compute(&data));
        println!("md5 = {}", md5); 
        if control.md5sum != md5 {
            anyhow::bail!(SignatureError::MD5(control.md5sum.to_string(), md5))
        }
    }
    
    if !control.sha1.is_empty() {
        let mut sha1 = Sha1::new();
        sha1.update(&data);
        let sha1 = format!("{:x}", sha1.finalize());
        println!("sha1 = {}", sha1);
        if *control.sha1 != sha1 {
            anyhow::bail!(SignatureError::SHA1(control.sha1.to_string(), sha1))
        }
    }
    
    if !control.sha256.is_empty() {
        let mut sha256 = Sha256::new();
        sha256.update(&data);
        let sha256 = format!("{:x}", sha256.finalize());
        println!("sha256 = {}", sha256);

        if *control.sha256 != sha256 {
            anyhow::bail!(SignatureError::SHA256(control.sha256.to_string(), sha256))
        }
    }

    if !control.sha512.is_empty() {
        let mut sha512 = Sha512::new();
        sha512.update(&data);
        let sha512 = format!("{:x}", sha512.finalize());
        println!("sha512 = {}", sha512);

        if *control.sha512 != sha512 {
            anyhow::bail!(SignatureError::SHA512(control.sha512.to_string(), sha512))
        }
    }


    Ok(())
}