use std::env;
use std::process;

use rpm::repos;
use repos::deb;

fn main() {
    let args: Vec<String> = env::args().collect();
    let package = &args[1];

    deb::extract(package).unwrap_or_else(|error| {
        println!("Failed to parse the file {} :: {}", package, error);
        process::exit(1);
    });
}