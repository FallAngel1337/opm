use std::fs;
use std::io::Error;

const DEB_REPOS: &str = "/etc/apt/sources.list";

#[derive(Debug, Clone)]
pub struct DebianSource {
    pub url: String,
    pub distribution: String,
    pub components: Vec<String>
}

impl DebianSource {
    pub fn new() -> Result<Vec<DebianSource>, Error> {
        let contents = fs::read_to_string(DEB_REPOS)?;
        let mut v: Vec<Self> = Vec::new();

        for d in contents.lines() {
            if d.starts_with("deb") && !d.contains("cdrom") {
                let split = d.split(' ').map(|e| e.to_owned()).collect::<Vec<_>>();
                let (mut url, distribution, components) = (split[1].to_owned(), split[2].to_owned(), split[3..].to_vec());

                if !url.ends_with('/') {
                    url.push('/');
                }

                v.push(
                    DebianSource {
                        url,
                        distribution,
                        components,
                    }
                );
            }
        }

        Ok (
            v
        )
    }
}