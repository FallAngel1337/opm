// Debian related modules
mod deb;

// General modules
mod install;
mod update;
mod cache;
mod setup;
mod remove;

pub mod config;
pub mod os_fingerprint;
pub mod errors;

pub use install::install;
pub use update::{update, clear};
pub use cache::{list_installed, search};
pub use setup::{setup, roll_back};
pub use remove::remove;