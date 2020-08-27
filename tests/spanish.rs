#[cfg(test)]
mod spanish_tests {
    use stop_words;

    const TARGET_FULL: &str = "spanish";
    const TARGET_ISO_632_1: &str = "es";
    const TARGET_ISO_632_2T: &str = "spa";

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