mod extract;
mod install;
mod download;
mod update;
mod dependencies;
mod signatures;
mod scripts;
mod remove;

pub mod cache;
pub mod package;
pub mod sources;

pub use install::install;
pub use update::{update, clear};
pub use cache::{db_dump, check_installed};
pub use remove::remove;

pub mod database {
    pub const DEBIAN_DATABASE: &str = "/var/lib/dpkg/status";
}