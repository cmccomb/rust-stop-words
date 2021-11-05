fn main() {
    // Get the stop words
    let x = stop_words::get("en");
    let y = stop_words::get(stop_words::LANGUAGE::English);

    // Print them
    for idx in 0..x.len() {
        assert_eq!(x[idx], y[idx])
    }
}
