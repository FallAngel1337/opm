use std::fs;
use std::io::Error;

const deb_repos: &'static str = "/etc/apt/sources.list"; // TODO: add from /etc/apt/sources.list.d/*

#[derive(Debug, Clone)]
pub struct DebianSource {
    pub url: String,
    pub distribution: String,
    pub components: Vec<String>
}

impl DebianSource {
    pub fn new() -> Result<Vec<DebianSource>, Error> {
        let contents = fs::read_to_string(deb_repos)?;
        let mut v: Vec<Self> = Vec::new();

        for d in contents.lines() {
            if !d.contains("#") && d.starts_with("deb") {
                let split = d.split(" ").collect::<Vec<&str>>().iter()
                    .map(|x| x.to_string()).collect::<Vec<String>>();
                v.push(
                    DebianSource {
                        url: split[1].clone(),
                        distribution: split[2].clone(),
                        components: Vec::from(&split[3..])
                    }
                );
            }
        }

        Ok (
            v
        )
    }
}