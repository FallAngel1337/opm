use std::collections::HashMap;
use std::fs;
use std::io::Error;

///
/// Kind of the package
///
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum PkgKind {
    Binary,
    Source,
}

///
/// Control file structures
///
#[derive(Debug, Clone)]
pub struct Paragraphs {
    fields: HashMap<String, String>
}

// Note: Maybe I need to optize this in the future
impl Paragraphs {
    pub fn new(ctrl_file: &str) -> Result<Self, Error> {
        let contents = fs::read_to_string(ctrl_file)?;

        let mut map: HashMap<String, String> = HashMap::new();

        for line in contents.lines() {
            let values = line.split(":").collect::<Vec<&str>>();
            map.insert(String::from(values[0]), String::from(values[1]));
        };

        Ok(
            Paragraphs {
                fields: map
            }
        )
    }
}

/// 
/// Debian binary package format structure
///
#[derive(Debug, Clone)]
pub struct DebPackage {
    pub control: Paragraphs,
    pub control_path: String,
    pub kind: PkgKind,
}

impl DebPackage {
    pub fn new(file: &str, kind: PkgKind) -> Result<Self, Error> {
        Ok(
            DebPackage {
                control: Paragraphs::new(file)?,
                control_path: String::from(file),
                kind
            }
        )
    }
}
