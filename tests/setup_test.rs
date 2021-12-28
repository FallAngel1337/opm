
#[cfg(test)]
mod setup_tests {
    use opm::repos::*;
    
    #[test]
    fn setup_from_file() {
        setup().unwrap();
    }
}