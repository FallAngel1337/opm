use std::env;
use std::process;

use rpm::repos;
use repos::install;

fn main() {
    let args: Vec<String> = env::args().collect();
    let package = &args[1];
    install(package);
}