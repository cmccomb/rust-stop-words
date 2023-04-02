use std::collections::HashMap;
use std::io::Write;

fn construct_json(languages: Vec<(&str, &str)>, file_name: &str) {
    // Make a json structure and populate it with file information
    let mut json_struct: HashMap<String, Vec<String>> = HashMap::new();
    for lingo in languages {
        let iso_code = lingo.0.to_string();
        let stop_word_set = lingo
            .1
            .split('\n')
            .map(String::from)
            .collect::<Vec<String>>();
        json_struct.insert(iso_code, stop_word_set);
    }

    // Convert to a JSON string
    let json_string: String = serde_json::to_string(&json_struct).unwrap();

    // Save the string
    let mut json_file =
        std::fs::File::create(std::env::var("OUT_DIR").unwrap() + "/" + file_name).unwrap();
    write!(json_file, "{json_string}").expect("Could not write JSON file.");
}

fn main() {
    // Update on changes
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/nltk");

    // Information on file locations
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

    // Information on file locations
    let constructed_languages = vec![
        ("dot", include_str!("src/constructed/dothraki")),
        ("dov", include_str!("src/constructed/dovahzul")),
        ("val", include_str!("src/constructed/highvalyrian")),
        ("tlh", include_str!("src/constructed/klingon")),
        ("qya", include_str!("src/constructed/quenya")),
        ("sjn", include_str!("src/constructed/sindarin")),
        ("nav", include_str!("src/constructed/navi")),
    ];

    construct_json(nltk_languages, "stopwords-nltk.json");
    construct_json(constructed_languages, "stopwords-constructed.json");
}
