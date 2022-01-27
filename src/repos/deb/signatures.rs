use anyhow::{self, Result};
use crate::repos::errors::SignatureError;
use sha2::{Sha256, Sha512, Digest};
use sha1::Sha1;
use super::package::DebPackage;

pub fn verify_sig<T: AsRef<[u8]>>(pkg: &DebPackage, data: T) -> Result<()> {
    let control = &pkg.control;
    
    if !control.md5sum.is_empty() {
        let md5 = format!("{:x}", md5::compute(&data));
        if control.md5sum != md5 {
            anyhow::bail!(SignatureError::MD5 { rs: control.md5sum.to_string(), ex: md5 });
        }
    }
    
    if !control.sha1.is_empty() {
        let mut sha1 = Sha1::new();
        sha1.update(&data);
        let sha1 = format!("{:x}", sha1.finalize());
        if *control.sha1 != sha1 {
            anyhow::bail!(SignatureError::SHA1 { rs: control.md5sum.to_string(), ex: sha1 });
        }
    }
    
    if !control.sha256.is_empty() {
        let mut sha256 = Sha256::new();
        sha256.update(&data);
        let sha256 = format!("{:x}", sha256.finalize());
        if *control.sha256 != sha256 {
            anyhow::bail!(SignatureError::SHA256 { rs: control.md5sum.to_string(), ex: sha256 })
        }
    }

    if !control.sha512.is_empty() {
        let mut sha512 = Sha512::new();
        sha512.update(&data);
        let sha512 = format!("{:x}", sha512.finalize());
        if *control.sha512 != sha512 {
            anyhow::bail!(SignatureError::SHA512 { rs: control.md5sum.to_string(), ex: sha512 })
        }
    }


    Ok(())
}