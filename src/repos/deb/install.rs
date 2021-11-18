use super::deb_control::DebControl;
use super::deb_package::DebPackage;
use super::extract::extract;

pub fn install(file: &str) {
    println!("installing ...");
    extract(file);
}