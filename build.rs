fn write_language(out: &mut String, match_arms: &mut Vec<String>, code: &str, words: &[&str]) {
    let constant_name = code.to_uppercase();
    out.push_str("static ");
    out.push_str(&constant_name);
    out.push_str(": [&str; ");
    out.push_str(&words.len().to_string());
    out.push_str("] = [");
    for word in words {
        out.push('"');
        out.push_str(&word.escape_default().to_string());
        out.push_str("\",");
    }
    out.push_str("];\n");
    match_arms.push(format!("        \"{code}\" => Some(&{constant_name}),\n"));
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/nltk");
    println!("cargo:rerun-if-changed=src/constructed");
    println!("cargo:rerun-if-changed=src/iso/stopwords-iso.json");

    let mut file_contents = String::new();
    let mut match_arms: Vec<String> = Vec::new();

    let nltk = std::env::var("CARGO_FEATURE_NLTK").is_ok();
    let constructed = std::env::var("CARGO_FEATURE_CONSTRUCTED").is_ok();
    let iso = std::env::var("CARGO_FEATURE_ISO").is_ok();

    let mut languages: std::collections::BTreeMap<
        std::string::String,
        std::vec::Vec<std::string::String>,
    > = std::collections::BTreeMap::new();

    if nltk {
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
        for (code, data) in nltk_languages {
            let words: std::vec::Vec<std::string::String> = data
                .split('\n')
                .filter(|w| !w.is_empty())
                .map(std::string::String::from)
                .collect();
            languages.insert(code.to_string(), words);
        }
    }

    if constructed {
        let constructed_languages = vec![
            ("dot", include_str!("src/constructed/dothraki")),
            ("dov", include_str!("src/constructed/dovahzul")),
            ("val", include_str!("src/constructed/highvalyrian")),
            ("tlh", include_str!("src/constructed/klingon")),
            ("qya", include_str!("src/constructed/quenya")),
            ("sjn", include_str!("src/constructed/sindarin")),
            ("nav", include_str!("src/constructed/navi")),
        ];
        for (code, data) in constructed_languages {
            let words: std::vec::Vec<std::string::String> = data
                .split('\n')
                .filter(|w| !w.is_empty())
                .map(std::string::String::from)
                .collect();
            languages.insert(code.to_string(), words);
        }
    }

    if iso {
        let json_data = std::fs::read_to_string("src/iso/stopwords-iso.json").unwrap();
        let parsed: std::collections::BTreeMap<
            std::string::String,
            std::vec::Vec<std::string::String>,
        > = serde_json::from_str(&json_data).unwrap();
        for (code, words_vec) in parsed {
            languages.entry(code).or_insert(words_vec);
        }
    }

    for (code, words_vec) in languages {
        let words: std::vec::Vec<&str> =
            words_vec.iter().map(std::string::String::as_str).collect();
        write_language(&mut file_contents, &mut match_arms, &code, &words);
    }

    file_contents.push_str(
        "pub(crate) fn lookup(language: &str) -> Option<&'static [&'static str]> {\n    match language {\n",
    );
    for arm in match_arms {
        file_contents.push_str(&arm);
    }
    file_contents.push_str("        _ => None,\n    }\n}\n");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = std::path::Path::new(&out_dir).join("stopwords.rs");
    std::fs::write(out_path, file_contents).unwrap();
}
