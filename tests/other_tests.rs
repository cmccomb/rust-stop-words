// use stop_words;

#[cfg(test)]
#[cfg(not(feature = "enum"))]
mod panic_tests {

    #[test]
    #[should_panic]
    fn bad_language_name() {
        let x = stop_words::get("engilsh");
        for y in x {
            println!("{}", y);
        }
    }

    #[test]
    #[should_panic]
    fn bad_language_code_1() {
        let x = stop_words::get("zz");
        for y in x {
            println!("{}", y);
        }
    }

    #[test]
    #[should_panic]
    fn bad_language_code_2t() {
        let x = stop_words::get("zzz");
        for y in x {
            println!("{}", y);
        }
    }
}
