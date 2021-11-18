/**
     * NAME="Ubuntu"
     * VERSION="20.04.3 LTS (Focal Fossa)"
     * ID=ubuntu
     * ID_LIKE=debian
     * PRETTY_NAME="Ubuntu 20.04.3 LTS"
     * VERSION_ID="20.04"
     * HOME_URL="https://www.ubuntu.com/"
     * SUPPORT_URL="https://help.ubuntu.com/"
     * BUG_REPORT_URL="https://bugs.launchpad.net/ubuntu/"
     * PRIVACY_POLICY_URL="https://www.ubuntu.com/legal/terms-and-policies/privacy-policy"
     * VERSION_CODENAME=focal
     * UBUNTU_CODENAME=focal
*/

use std::collections::HashMap;
use std::fs;
use std::io::Error;

///
/// Struct to indetify if it's a Debian(-based) distro
/// by reading the `/etc/os-release` file
///
#[derive(Debug, Clone)]
struct OsRelease {
    name: String,
    id_like: String,
}

// Note: Maybe I need to optize this in the future
impl OsRelease {
    fn new() -> Result<Self, Error> {
        let contents = match fs::read_to_string("/etc/os-release") {
            Ok(f) => f,
            Err(_) => {
                fs::read_to_string("/etc/centos-release")?
            }
        };
        let mut map: HashMap<String, String> = HashMap::new();

        for line in contents.lines() {
            let values = line.split("=").collect::<Vec<&str>>();
            map.insert(String::from(values[0]), String::from(values[1]));
        };

        Ok(
            OsRelease {
                name: map.get("NAME").unwrap().to_string(),
                id_like: map.get("ID_LIKE").unwrap().to_string(),
            }
        )
    }
}

#[derive(Debug)]
pub enum Distribution {
    Debian,
    Rhel,
    Other,
}

impl Distribution {
    pub fn get_distro() -> Self {
        let os_rel = OsRelease::new().unwrap();
        if let Some(_) = os_rel.id_like.find("debian") {
            Distribution::Debian
        } else if let Some(_) = os_rel.id_like.find("rhel") {
            Distribution::Rhel
        } else {
            Distribution::Other
        }
    }
}

