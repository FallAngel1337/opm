///
/// Debian package install
/// 

// use super::deb_package::DebPackage;
use super::extract::extract;

pub fn install(file: &str) {
    println!("installing ...");
    println!("package: {:?}", extract(file).unwrap());

}