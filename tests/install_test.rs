#[cfg(test)]
mod install_tests {
    use opm::repos::*;
    
    #[test]
    #[should_panic]
    fn install_not_found_deb_install() {
        let mut config = config::Config::new("deb").unwrap();
        install(&mut config, "iNvAlIdPaCkAgE.deb").unwrap();
    }
}