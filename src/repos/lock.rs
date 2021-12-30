#[inline]
pub fn lock() -> anyhow::Result<()> {
    std::fs::read("/etc/shadow")?
}