extern "C" {
    pub fn getuid() -> u32;
}

mod repos;

pub use repos::{setup, roll_back};
pub use repos::install;
pub use repos::{update, clear};
pub use repos::{list_installed, search};
pub use repos::remove;