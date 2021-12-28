#[cfg(test)]
mod install_tests {
    use opm::repos::*;

    #[test]
    fn install_not_found_test() {
        let mut config = config::Config::new("deb").unwrap();
        install(&mut config, "iNvAlIdPaCkAgE").unwrap_err();
    }
    
    #[test]
    #[should_panic]
    fn install_not_found_deb_install() {
        let mut config = config::Config::new("deb").unwrap();
        install(&mut config, "iNvAlIdPaCkAgE.deb").unwrap();
    }
    
    #[test]
    fn install_already_installed() {
        let mut config = config::Config::new("deb").unwrap();
        install(&mut config, "dpkg").unwrap_err();
    }
}