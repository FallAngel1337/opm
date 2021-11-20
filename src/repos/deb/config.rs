use std::fs;
use std::io::Error;

const deb_repos: &'static str = "/etc/apt/sources.list"; // TODO: add from /etc/apt/sources.list.d/*

#[derive(Debug, Clone)]
pub struct ReposList {
    pub list: Vec<String>
}

impl ReposList {
    pub fn new() -> Result<Self, Error>{
        let contents = fs::read_to_string(deb_repos)?;
        let mut v: Vec<String> = Vec::new();

        for d in contents.lines() {
            if !d.contains("#") && d.starts_with("deb") {
                let split = d.split(" ").collect::<Vec<&str>>();
                v.push(split[1].to_string());
            }
        }

        Ok (
            ReposList {
                list: v
            }
        )
    }
}