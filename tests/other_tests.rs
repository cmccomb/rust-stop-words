#[cfg(test)]
mod panic_tests {
    #[test]
    #[should_panic]
    fn bad_language_code_1() {
        let x = stop_words::get("zz");
        for y in x {
            println!("{}", y);
        }
    }
}