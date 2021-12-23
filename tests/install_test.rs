use opm::repos;

#[test]
#[should_panic]
fn install_not_found_test() {
    let mut config = repos::config::Config::new("deb").unwrap();
    repos::install(&mut config, "iNvAlIdPaCkAgE").unwrap_err();
}

#[test]
#[should_panic]
fn install_not_found_deb_install() {
    let mut config = repos::config::Config::new("deb").unwrap();
    repos::install(&mut config, "iNvAlIdPaCkAgE.deb").unwrap();
}

#[test]
fn install_already_installed() {
    let mut config = repos::config::Config::new("deb").unwrap();
    repos::install(&mut config, "gcc").unwrap_err(); // Chagne this to a package you do have installed
}