// Debian related modules
mod deb;
mod database;

// General modules
mod install;
pub mod config;
pub mod utils;
pub mod errors;

pub use install::install;