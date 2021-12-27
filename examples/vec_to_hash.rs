use std::collections::HashSet;

fn main() {
    // Get the stop words
    let vec = stop_words::get(stop_words::LANGUAGE::English);

    // Convert to hash set
    let set: HashSet<String> = vec.into_iter().collect();
}
