#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! # About
//! Stop words are words that don't carry much meaning, and are typically removed as a preprocessing step before text
//! analysis or natural language processing. This crate contains common stop words for a variety of languages. This crate uses stop word
//! lists from [this resource](https://github.com/Alir3z4/stop-words/tree/bd8cc1434faeb3449735ed570a4a392ab5d35291) and also from [NLTK](https://www.nltk.org/).
//!
//! This crate currently includes the following languages:
//! - Arabic
//! - Azerbaijani
//! - Bulgarian
//! - Catalan
//! - Czech
//! - Danish
//! - Dutch
//! - English
//! - Finnish
//! - French
//! - German
//! - Greek
//! - Hebrew
//! - Hindi
//! - Hungarian
//! - Indonesian
//! - Italian
//! - Kazakh
//! - Nepali
//! - Norwegian
//! - Polish
//! - Portuguese
//! - Romanian
//! - Russian
//! - Slovak
//! - Slovenian
//! - Spanish
//! - Swedish
//! - Tajik
//! - Turkish
//! - Ukrainian
//! - Vietnamese

use std::collections::HashSet;

/// Constant containing an array of available language names, spelled out
pub const LANGUAGES: [&str; 32] = ["arabic", "azerbaijani", "catalan", "danish", "english", "french",
    "hindi", "indonesian", "norwegian", "portuguese", "russian", "spanish", "turkish", "vietnamese",
    "bulgarian", "czech", "dutch", "finnish", "german", "hungarian", "italian", "polish",
    "romanian", "slovak", "swedish", "ukrainian", "hebrew", "greek", "kazakh", "nepali", "slovenian",
    "tajik"];

/// Constant containing an array of available language names, using ISO-693-1 codes
pub const LANGUAGES_ISO_693_1: [&str; 32] = ["ar", "az", "ca", "da", "en", "fr",
    "hi", "in", "nn", "pt", "ru", "es", "tr", "vi",
    "bg", "cs", "nl", "fi", "de", "hu", "it", "pl",
    "ro", "sk", "sv", "uk", "he", "el", "kk", "ne", "sl",
    "tg"];

/// Constant containing an array of available language names, using ISO-693-2T codes
pub const LANGUAGES_ISO_693_2T: [&str; 32] = ["ara", "aze", "cat", "dan", "eng", "fra",
    "hin", "ind", "nno", "por", "rus", "spa", "tur", "vie",
    "bul", "ces", "nld", "fin", "deu", "hun", "ita", "pol",
    "ron", "slk", "swe", "ukr", "heb", "ell", "kaz", "nep", "slv",
    "tgk"];

/// The only function you'll ever need! Given a language code or name it returns common stop words as a ``Vec<String>``
///
/// ```
/// let vec = stop_words::get("spanish");
/// ```
pub fn get(target_language: &str) -> Vec<String> {
    // Match the full language name
    match get_language_from_code(target_language) {
        "english" =>    read_from_bytes(include_bytes!("savand/english.txt")),
        "hebrew" =>     read_from_bytes(include_bytes!("savand/hebrew.txt")),
        "arabic" =>     read_from_bytes(include_bytes!("savand/arabic.txt")),
        "catalan" =>    read_from_bytes(include_bytes!("savand/catalan.txt")),
        "danish" =>     read_from_bytes(include_bytes!("savand/danish.txt")),
        "french" =>     read_from_bytes(include_bytes!("savand/french.txt")),
        "hindi" =>      read_from_bytes(include_bytes!("savand/hindi.txt")),
        "indonesian" => read_from_bytes(include_bytes!("savand/indonesian.txt")),
        "norwegian" =>  read_from_bytes(include_bytes!("savand/norwegian.txt")),
        "portuguese" => read_from_bytes(include_bytes!("savand/portuguese.txt")),
        "russian" =>    read_from_bytes(include_bytes!("savand/russian.txt")),
        "spanish" =>    read_from_bytes(include_bytes!("savand/spanish.txt")),
        "turkish" =>    read_from_bytes(include_bytes!("savand/turkish.txt")),
        "vietnamese" => read_from_bytes(include_bytes!("savand/vietnamese.txt")),
        "bulgarian" =>  read_from_bytes(include_bytes!("savand/bulgarian.txt")),
        "czech" =>      read_from_bytes(include_bytes!("savand/czech.txt")),
        "dutch" =>      read_from_bytes(include_bytes!("savand/dutch.txt")),
        "finnish" =>    read_from_bytes(include_bytes!("savand/finnish.txt")),
        "german" =>     read_from_bytes(include_bytes!("savand/german.txt")),
        "hungarian" =>  read_from_bytes(include_bytes!("savand/hungarian.txt")),
        "italian" =>    read_from_bytes(include_bytes!("savand/italian.txt")),
        "polish" =>     read_from_bytes(include_bytes!("savand/polish.txt")),
        "romanian" =>   read_from_bytes(include_bytes!("savand/romanian.txt")),
        "slovak" =>     read_from_bytes(include_bytes!("savand/slovak.txt")),
        "swedish" =>    read_from_bytes(include_bytes!("savand/swedish.txt")),
        "ukrainian" =>  read_from_bytes(include_bytes!("savand/ukrainian.txt")),
        "azerbaijani" =>read_from_bytes(include_bytes!("nltk/azerbaijani")),
        "greek" =>      read_from_bytes(include_bytes!("nltk/greek")),
        "kazakh" =>     read_from_bytes(include_bytes!("nltk/kazakh")),
        "nepali" =>     read_from_bytes(include_bytes!("nltk/nepali")),
        "slovenian" =>  read_from_bytes(include_bytes!("nltk/slovene")),
        "tajik" =>      read_from_bytes(include_bytes!("nltk/tajik")),
        _ =>            panic!("Unfortunately, the {} language is not currently supported. Please make sure that the name of the language is spelled in English.", target_language)
    }
}

/// Ok, you might need this function too. It fetches stop words specifically for NLTK.
///
/// ```
/// let vec = stop_words::get_nltk("spanish");
/// ```
pub fn get_nltk(target_language: &str) -> Vec<String> {
    // Match the full language name
    match get_language_from_code(target_language) {
        "english" =>    read_from_bytes(include_bytes!("nltk/english")),
        "arabic" =>     read_from_bytes(include_bytes!("nltk/arabic")),
        "danish" =>     read_from_bytes(include_bytes!("nltk/danish")),
        "french" =>     read_from_bytes(include_bytes!("nltk/french")),
        "indonesian" => read_from_bytes(include_bytes!("nltk/indonesian")),
        "norwegian" =>  read_from_bytes(include_bytes!("nltk/norwegian")),
        "portuguese" => read_from_bytes(include_bytes!("nltk/portuguese")),
        "russian" =>    read_from_bytes(include_bytes!("nltk/russian")),
        "spanish" =>    read_from_bytes(include_bytes!("nltk/spanish")),
        "turkish" =>    read_from_bytes(include_bytes!("nltk/turkish")),
        "greek" =>      read_from_bytes(include_bytes!("nltk/greek")),
        "dutch" =>      read_from_bytes(include_bytes!("nltk/dutch")),
        "finnish" =>    read_from_bytes(include_bytes!("nltk/finnish")),
        "german" =>     read_from_bytes(include_bytes!("nltk/german")),
        "hungarian" =>  read_from_bytes(include_bytes!("nltk/hungarian")),
        "italian" =>    read_from_bytes(include_bytes!("nltk/italian")),
        "romanian" =>   read_from_bytes(include_bytes!("nltk/romanian")),
        "swedish" =>    read_from_bytes(include_bytes!("nltk/swedish")),
        "azerbaijani" =>read_from_bytes(include_bytes!("nltk/azerbaijani")),
        "kazakh" =>     read_from_bytes(include_bytes!("nltk/kazakh")),
        "nepali" =>     read_from_bytes(include_bytes!("nltk/nepali")),
        "slovenian" =>  read_from_bytes(include_bytes!("nltk/slovene")),
        "tajik" =>      read_from_bytes(include_bytes!("nltk/tajik")),
        _ =>            panic!("Unfortunately, the {} language is not currently supported in NLTK. Please make sure that the name of the language is spelled in English.", target_language)
    }
}

/// This function takes an arbitrary code and converts it as needed to a full language name
fn get_language_from_code(code: &str) -> &str {
    if code.len() == 2 {
        get_language_from_iso(code, LANGUAGES_ISO_693_1)
    } else if code.len() == 3 {
        get_language_from_iso(code, LANGUAGES_ISO_693_2T)
    } else {
        code
    }
}

/// This function converts a language code to a full language name
fn get_language_from_iso<'a>(code: &'a str, library: [&str; 32]) -> &'a str {
    let mut iter = library.iter();
    let idx = iter.position(|&x| x == code);
    match idx {
        Some(x) => LANGUAGES[x],
        None => panic!("It looks like you're trying to use an ISO 693-1 (2-letter) language code. Unfortunately, the {} language code is not currently supported.", code),
    }
}

/// This function converts the bytestring to a vector
fn read_from_bytes(bytes: &[u8]) -> Vec<String> {
    let contents=String::from_utf8_lossy(bytes);
    let split_contents = contents.split('\n');
    let mut output = vec![];
    for word in split_contents {
        output.push(String::from(word));
    }
    output
}

/// This function converts the standard ``Vec<String>`` output to a ``HashSet<String>``
///
/// ```
/// let vec = stop_words::get("nl");
/// let set = stop_words::vec_to_set(vec);
/// ```
pub fn vec_to_set(words: Vec<String>) -> HashSet<String> {
    let mut hash_words: HashSet<String> = HashSet::new();
    for word in words {
        hash_words.insert(word);
    }
    hash_words
}


/// Just a few tests here
#[cfg(test)]
mod conversion_tests {
    use crate as stop_words;

    #[test]
    fn convert_to_set() {
        let vec = stop_words::get("es");
        let set = stop_words::vec_to_set(vec);
        for y in set {
            println!("{}", y);
        }
    }
}

#[cfg(test)]
mod panic_tests {
    use crate as stop_words;

    #[test]
    #[should_panic]
    fn bad_language_name() {
        let x = stop_words::get("engilsh");
        for y in x {
            println!("{}", y);
        }
    }

    #[test]
    #[should_panic]
    fn bad_language_code_1() {
        let x = stop_words::get("zz");
        for y in x {
            println!("{}", y);
        }
    }

    #[test]
    #[should_panic]
    fn bad_language_code_2t() {
        let x = stop_words::get("zzz");
        for y in x {
            println!("{}", y);
        }
    }
}