#[cfg(test)]
mod portuguese_tests {
    use stop_words;

    const TARGET_FULL: &str = "portuguese";
    const TARGET_ISO_632_1: &str = "pt";
    const TARGET_ISO_632_2T: &str = "por";

    #[test]
    fn compare_full_to_2letter() {
        let x = stop_words::get(TARGET_FULL);
        let y = stop_words::get(TARGET_ISO_632_1);
        for idx in 0..x.len() {
            assert_eq!(x[idx], y[idx])
        }
    }

    #[test]
    fn compare_full_to_3letter() {
        let x = stop_words::get(TARGET_FULL);
        let y = stop_words::get(TARGET_ISO_632_2T);
        for idx in 0..x.len() {
            assert_eq!(x[idx], y[idx])
        }
    }

    #[test]
    fn check_nltk() {
        let x = stop_words::get_nltk(TARGET_FULL);
        for idx in 0..x.len() {
            println!("{}", x[idx])
        }
    }
}