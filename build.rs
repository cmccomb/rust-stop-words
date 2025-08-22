fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/iso/stopwords-iso.json");
    println!("cargo:rerun-if-changed=src/nltk");
    println!("cargo:rerun-if-changed=src/constructed");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("stopwords.rs");
    let mut file = std::fs::File::create(dest_path).unwrap();

    <std::fs::File as std::io::Write>::write_all(
        &mut file,
        b"pub fn stop_words(lang: &str) -> Option<&'static [&'static str]> {\n    match lang {\n",
    )
    .unwrap();

    if std::env::var("CARGO_FEATURE_NLTK").is_ok() {
        let languages = vec![
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
        for (code, words) in languages {
            <std::fs::File as std::io::Write>::write_all(
                &mut file,
                format!("        \"{code}\" => Some(&[").as_bytes(),
            )
            .unwrap();
            for word in words.lines() {
                <std::fs::File as std::io::Write>::write_all(
                    &mut file,
                    format!("{word:?},").as_bytes(),
                )
                .unwrap();
            }
            <std::fs::File as std::io::Write>::write_all(&mut file, b"]),\n").unwrap();
        }
    } else if std::env::var("CARGO_FEATURE_CONSTRUCTED").is_ok() {
        let languages = vec![
            ("dot", include_str!("src/constructed/dothraki")),
            ("dov", include_str!("src/constructed/dovahzul")),
            ("val", include_str!("src/constructed/highvalyrian")),
            ("tlh", include_str!("src/constructed/klingon")),
            ("qya", include_str!("src/constructed/quenya")),
            ("sjn", include_str!("src/constructed/sindarin")),
            ("nav", include_str!("src/constructed/navi")),
        ];
        for (code, words) in languages {
            <std::fs::File as std::io::Write>::write_all(
                &mut file,
                format!("        \"{code}\" => Some(&[").as_bytes(),
            )
            .unwrap();
            for word in words.lines() {
                <std::fs::File as std::io::Write>::write_all(
                    &mut file,
                    format!("{word:?},").as_bytes(),
                )
                .unwrap();
            }
            <std::fs::File as std::io::Write>::write_all(&mut file, b"]),\n").unwrap();
        }
    } else {
        let data: std::collections::BTreeMap<String, Vec<String>> =
            serde_json::from_str(&std::fs::read_to_string("src/iso/stopwords-iso.json").unwrap())
                .unwrap();
        for (code, list) in data {
            <std::fs::File as std::io::Write>::write_all(
                &mut file,
                format!("        \"{code}\" => Some(&[").as_bytes(),
            )
            .unwrap();
            for word in list {
                <std::fs::File as std::io::Write>::write_all(
                    &mut file,
                    format!("{word:?},").as_bytes(),
                )
                .unwrap();
            }
            <std::fs::File as std::io::Write>::write_all(&mut file, b"]),\n").unwrap();
        }
    }

    <std::fs::File as std::io::Write>::write_all(&mut file, b"        _ => None,\n    }\n}\n")
        .unwrap();
}
