use std::collections::HashSet;

fn main() {
    // Get the stop words

    let vec = stop_words::get(stop_words::LANGUAGE::English);
    let set: HashSet<String> = vec.into_iter().collect();

    // Print them
    for word in set {
        println!("{}", word)
    }
}
