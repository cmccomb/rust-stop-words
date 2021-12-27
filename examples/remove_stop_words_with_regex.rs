use stop_words::{get, LANGUAGE};
use human_regex::{exactly, or, punctuation, whitespace, zero_or_more, word_boundary, one_or_more};

fn main() {
    // Read in a file
    let mut text = std::fs::read_to_string("foreword.txt").expect("Cannot read file");

    // Print the contents
    println!("Original text:\n{}", text);

    // Get the stopwords
    let words = get(LANGUAGE::English);

    // Remove punctuation and lowercase the text to make parsing easier
    let text = text.to_ascii_lowercase();
    let regex_for_punctuation = one_or_more(punctuation());
    let text_without_punctuation = regex_for_punctuation.to_regex().replace_all(&*text, "");

    // Make a regex to match stopwords with trailing spaces and punctuation
    let regex_for_stop_words = word_boundary() + exactly(1, or(&words)) + word_boundary() + one_or_more(whitespace());

    // Remove stop words
    let clean_text = regex_for_stop_words.to_regex().replace_all(&*text_without_punctuation, "");
    println!("\nClean text:\n{}", clean_text);

}
