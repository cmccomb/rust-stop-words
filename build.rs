use std::collections::HashMap;
use std::io::Write;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/nltk");

    let nltk_languages = vec![
        ("ar", include_str!("src/nltk/arabic")),
        ("az", include_str!("src/nltk/azerbaijani")),
        ("da", include_str!("src/nltk/danish")),
        ("nl", include_str!("src/nltk/dutch")),
        ("en", include_str!("src/nltk/english")),
        ("fi", include_str!("src/nltk/finnish")),
        ("fr", include_str!("src/nltk/french")),
        ("de", include_str!("src/nltk/german")),
        ("el", include_str!("src/nltk/greek")),
        ("hu", include_str!("src/nltk/hungarian")),
        ("id", include_str!("src/nltk/indonesian")),
        ("it", include_str!("src/nltk/italian")),
        ("kk", include_str!("src/nltk/kazakh")),
        ("ne", include_str!("src/nltk/nepali")),
        ("no", include_str!("src/nltk/norwegian")),
        ("pt", include_str!("src/nltk/portuguese")),
        ("ro", include_str!("src/nltk/romanian")),
        ("ru", include_str!("src/nltk/russian")),
        ("sl", include_str!("src/nltk/slovenian")),
        ("es", include_str!("src/nltk/spanish")),
        ("sv", include_str!("src/nltk/swedish")),
        ("tg", include_str!("src/nltk/tajik")),
        ("tr", include_str!("src/nltk/turkish")),
    ];

    let mut json_struct: HashMap<String, Vec<String>> = HashMap::new();
    for lingo in nltk_languages {
        let iso_code = lingo.0.to_string();
        let stop_word_set = lingo
            .1
            .split('\n')
            .map(String::from)
            .collect::<Vec<String>>();
        json_struct.insert(iso_code, stop_word_set);
    }

    let nltk_stop_words: String = serde_json::to_string(&json_struct).unwrap();

    let mut nltk_file =
        std::fs::File::create(std::env::var("OUT_DIR").unwrap() + "/nltk_file.json").unwrap();

    write!(nltk_file, "{nltk_stop_words}").expect("Could not write NLTK JSON file.");
}
