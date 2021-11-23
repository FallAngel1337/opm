use std::path::Path;

/// Default Debian's package manager
const DEB_PKG: &[&str] = &["/usr/bin/apt", "/usr/bin/dpkg"];

#[derive(Debug)]
pub enum Distribution {
    Debian,
    Rhel,
    Other,
}

impl Distribution {
    pub fn get_distro() -> Self {
        if Path::new(DEB_PKG[0]).exists() || Path::new(DEB_PKG[1]).exists() {
            return Self::Debian;
        }

        // do it the smame for others

        Self::Other
    }
}

