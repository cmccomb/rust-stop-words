// This example demonstrates removing stop words using regular expressions.
// It is only built when ISO or NLTK stop words are available and constructed languages are disabled.
#[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
fn main() {
    // Read in a file
    let document = std::fs::read_to_string("examples/foreword.txt").expect("Cannot read file");

    // Print the contents
    println!("Original text:\n{}", document);

    // Get the stopwords
    let words = stop_words::get(stop_words::LANGUAGE::English);

    // Remove punctuation and lowercase the text to make parsing easier
    let lowercase_doc = document.to_ascii_lowercase();
    let regex_for_punctuation = human_regex::one_or_more(human_regex::punctuation());
    let text_without_punctuation = regex_for_punctuation
        .to_regex()
        .replace_all(&lowercase_doc, "");

    // Make a regex to match stopwords with trailing spaces and punctuation
    let regex_for_stop_words = human_regex::word_boundary()
        + human_regex::exactly(1, human_regex::or(&words))
        + human_regex::word_boundary()
        + human_regex::one_or_more(human_regex::whitespace());

    // Remove stop words
    let clean_text = regex_for_stop_words
        .to_regex()
        .replace_all(&text_without_punctuation, "");
    println!("\nClean text:\n{}", clean_text);
}

#[cfg(any(
    all(not(feature = "nltk"), not(feature = "iso")),
    feature = "constructed"
))]
fn main() {}
