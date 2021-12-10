use std::env;

const PKG_FMT: &'static str = "PKG_FMT"; // The package format; It could be .deb, .rpm, etc

#[derive(Debug)]
pub enum PackageFormat {
    Deb,
    Rpm,
    Other,
}

impl PackageFormat {
    pub fn get_format() -> Option<Self> {
        if let Ok(pkg_fmt) = env::var(PKG_FMT) {
            match pkg_fmt.trim().to_lowercase() {
                "deb" => Self::Deb,
                "rpm" => Self::Rpm,
                _ => Self::Other
            }
        } else {
            None
        }
    }
}

