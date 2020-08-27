use std::collections::HashSet;
use stop_words;

fn main() {
    // Get the stop words
    let vec = stop_words::get("english");
    let set: HashSet<String> = vec.into_iter().collect();

    // Print them
    for word in set {
        println!("{}", word)
    }
}
