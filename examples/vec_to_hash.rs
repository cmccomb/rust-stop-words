use std::collections::HashSet;
use stop_words;

fn main() {
    // Get the stop words

    #[cfg(not(feature = "enum"))]
    let vec = stop_words::get("english");
    #[cfg(feature = "enum")]
    let vec = stop_words::get(stop_words::LANGUAGE::English);
    let set: HashSet<String> = vec.into_iter().collect();

    // Print them
    for word in set {
        println!("{}", word)
    }
}
