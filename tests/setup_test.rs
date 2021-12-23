use opm::repos;

#[test]
fn setup_test() {
    repos::setup().unwrap();
}