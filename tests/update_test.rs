use opm::repos;

#[test]
fn update_test() {
    let mut config = repos::config::Config::new("deb").unwrap();
    repos::update(&mut config).unwrap()
}