// Debian related modules
mod deb;
mod database;

// General modules
mod install;
mod update;

pub mod config;
pub mod utils;
pub mod errors;

pub use install::install;
pub use update::update;