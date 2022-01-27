// Debian related modules
mod deb;

// General modules
mod commands;

pub use commands::install::install;
pub use commands::search::{search, list_installed};
pub use commands::remove::remove;
pub use commands::update::{clear, update};
pub use setup::{setup, roll_back};
pub mod os_fingerprint;

pub mod config;
pub mod packages;
pub mod errors;
pub mod setup;