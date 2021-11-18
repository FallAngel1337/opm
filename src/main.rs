use std::env;
use std::process;

use rpm::repos;

fn main() {
    let args: Vec<String> = env::args().collect();
    let package = &args[1];
    
    repos::install(package);
}