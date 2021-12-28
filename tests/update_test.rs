
#[cfg(test)]
mod update_test {
    use opm::repos::*;
    
    #[test]
    fn update_test() {
        let mut config = config::Config::new("deb").unwrap();
        update(&mut config).unwrap()
    }
}