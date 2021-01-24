use stop_words;

fn main() {
    // Get the stop words
    #[cfg(not(feature = "enum"))]
    let words = stop_words::get("english");
    #[cfg(feature = "enum")]
    let words = stop_words::get(stop_words::LANGUAGE::English);

    // Print them
    for word in words {
        println!("{}", word)
    }
}
