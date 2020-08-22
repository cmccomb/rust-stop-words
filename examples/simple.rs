use stop_words;

fn main() {
    let words = stop_words::get("english");
    for word in words {
        println!("{}", word)
    }
}