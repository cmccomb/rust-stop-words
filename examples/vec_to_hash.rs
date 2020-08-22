use stop_words;

fn main() {
    // Get the stop words
    let vec = stop_words::get("english");
    let set = stop_words::vec_to_hash(vec);

    // Print them
    for word in set {
        println!("{}", word)
    }
}