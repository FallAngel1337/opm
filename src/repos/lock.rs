#[inline]
pub fn lock() -> anyhow::Result<()> {
    let _ = std::fs::read("/etc/shadow")?;
    Ok(())
}