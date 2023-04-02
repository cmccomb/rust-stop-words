#[cfg(not(feature = "constructed"))]
#[cfg(test)]
mod panic_tests {
    #[test]
    #[should_panic]
    fn bad_language_code_1() {
        let x = stop_words::get("dothraki");
        for y in x {
            println!("{}", y);
        }
    }
}
