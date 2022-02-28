#[cfg(target_family="unix")]
extern "C" {
    pub fn getuid() -> u32;
}

pub fn user_input(msg: &str) -> anyhow::Result<()> {
    let mut answer = String::new();
    print!("{}", msg);
    std::io::Write::flush(&mut std::io::stdout())?;
    std::io::stdin().read_line(&mut answer)?;

    if answer.to_ascii_lowercase().trim().starts_with('y') {
        Ok(())
    } else {
        eprintln!("Exiting installation process...");
        anyhow::bail!(repos::errors::InstallError::UserInterrupt);
    }
}

mod repos;

pub use repos::{setup, roll_back};
pub use repos::install;
pub use repos::{update, clear};
pub use repos::{list_installed, search};
pub use repos::remove;