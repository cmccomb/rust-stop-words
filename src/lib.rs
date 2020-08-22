#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! This module contains stop words for a number of languages, using data from [this resource](https://github.com/Alir3z4/stop-words/tree/bd8cc1434faeb3449735ed570a4a392ab5d35291)
//! ```
//! use stop_words;
//! let x = stop_words::get("english");
//! ```

/// Constant containing an array of available language names, spelled out
pub const LANGUAGES: [&str; 26] = ["arabic", "catalan", "danish", "english", "french", "hindi",
    "indonesian", "norwegian", "portuguese", "russian", "spanish", "turkish", "vietnamese",
    "bulgarian", "czech", "dutch", "finnish", "german", "hungarian", "italian", "polish",
    "romanian", "slovak", "swedish", "ukrainian", "hebrew"];

/// Constant containing an array of available language names, using ISO-693-1 codes
const LANGUAGES_ISO_693_1: [&str; 26] = ["ar", "ca", "da", "en", "fr", "hi",
    "in", "nn", "pt", "ru", "es", "tr", "vi",
    "bg", "cs", "nl", "fi", "de", "hu", "it", "pl",
    "ro", "sk", "sv", "uk", "he"];

/// Constant containing an array of available language names, using ISO-693-1 codes
const LANGUAGES_ISO_693_2T: [&str; 26] = ["ara", "cat", "dan", "eng", "fra", "hin",
    "ind", "nno", "por", "rus", "spa", "tur", "vie",
    "bul", "ces", "nld", "fin", "deu", "hun", "ita", "pol",
    "ron", "slk", "swe", "ukr", "heb"];

/// The only function you'll ever need! Given a language code it returns common stop words
pub fn get(language: &str) -> Vec<String> {
    // Check to see if its an ISO code, and if so
    let new_language= if language.len() == 2 {
        convert_language_from_iso_693_1(language)
    } else if language.len() == 3 {
        convert_language_from_iso_693_2t(language)
    } else {
        language
    };

    // Match the full language name
    match new_language {
        "english" => read_from_bytes(include_bytes!("english.txt")),
        "hebrew" => read_from_bytes(include_bytes!("hebrew.txt")),
        "arabic" => read_from_bytes(include_bytes!("arabic.txt")),
        "catalan" => read_from_bytes(include_bytes!("catalan.txt")),
        "danish" => read_from_bytes(include_bytes!("danish.txt")),
        "french" => read_from_bytes(include_bytes!("french.txt")),
        "hindi" => read_from_bytes(include_bytes!("hindi.txt")),
        "indonesian" => read_from_bytes(include_bytes!("indonesian.txt")),
        "norwegian" => read_from_bytes(include_bytes!("norwegian.txt")),
        "portugese" => read_from_bytes(include_bytes!("portuguese.txt")),
        "russian" => read_from_bytes(include_bytes!("russian.txt")),
        "spanish" => read_from_bytes(include_bytes!("spanish.txt")),
        "turkish" => read_from_bytes(include_bytes!("turkish.txt")),
        "vietnamese" => read_from_bytes(include_bytes!("vietnamese.txt")),
        "bulgarian" => read_from_bytes(include_bytes!("bulgarian.txt")),
        "czech" => read_from_bytes(include_bytes!("czech.txt")),
        "dutch" => read_from_bytes(include_bytes!("dutch.txt")),
        "finnish" => read_from_bytes(include_bytes!("finnish.txt")),
        "german" => read_from_bytes(include_bytes!("german.txt")),
        "hungarian" => read_from_bytes(include_bytes!("hungarian.txt")),
        "italian" => read_from_bytes(include_bytes!("italian.txt")),
        "polish" => read_from_bytes(include_bytes!("polish.txt")),
        "romanian" => read_from_bytes(include_bytes!("romanian.txt")),
        "slovak" => read_from_bytes(include_bytes!("slovak.txt")),
        "swedish" => read_from_bytes(include_bytes!("swedish.txt")),
        "ukrainian" => read_from_bytes(include_bytes!("ukrainian.txt")),
        _ => panic!("It looks like you're trying to spell out a full language name. Unfortunately, the {} language is not currently supported. Please make sure that the name of the language is spelled in English", language)
    }
}

/// This function converts the ISO-693-1 language string to a full name
fn convert_language_from_iso_693_1(code: &str) -> &str {
    let mut iter = LANGUAGES_ISO_693_1.iter();
    let idx = iter.position(|&x| x == code);
    match idx {
        Some(x) => LANGUAGES[x],
        None => panic!("It looks like you're trying to use an ISO 693-1 (2-letter) language code. Unfortunately, the {} language code is not currently supported.", code),
    }
}

/// This function converts the ISO-693-1 language string to a full name
fn convert_language_from_iso_693_2t(code: &str) -> &str {
    let mut iter = LANGUAGES_ISO_693_2T.iter();
    let idx = iter.position(|&x| x == code);
    match idx {
        Some(x) => LANGUAGES[x],
        None => panic!("It looks like you're trying to use an ISO 693-2T (3-letter) language code. Unfortunately, the {} language code is not currently supported.", code),
    }
}

/// This function converts the bytestring to a vector
fn read_from_bytes(bytes: &[u8]) -> Vec<String> {
    let contents=String::from_utf8_lossy(bytes);
    let split_contents = contents.split("\n");
    let mut output = vec![];
    for word in split_contents {
        output.push(String::from(word));
    }
    output
}

// TODO: More tests
#[cfg(test)]
mod tests {
    use crate::get;

    #[test]
    fn good_language_name() {
        let x = get("arabic");
        for y in x {
            println!("{}", y);
        }
    }

    #[test]
    fn good_language_code_1() {
        let x = get("en");
        for y in x {
            println!("{}", y);
        }
    }

    #[test]
    fn good_language_code_2t() {
        let x = get("eng");
        for y in x {
            println!("{}", y);
        }
    }

    #[test]
    fn hebrew() {
        let x = get("hebrew");
        for y in x {
            println!("{}", y);
        }
    }

    #[test]
    fn arabic() {
        let x = get("arabic");
        for y in x {
            println!("{}", y);
        }
    }

    #[test]
    fn russian() {
        let x = get("russian");
        for y in x {
            println!("{}", y);
        }
    }

    #[test]
    #[should_panic]
    fn bad_language_name() {
        let x = get("engilsh");
        for y in x {
            println!("{}", y);
        }
    }

    #[test]
    #[should_panic]
    fn bad_language_code_1() {
        let x = get("zz");
        for y in x {
            println!("{}", y);
        }
    }

    #[test]
    #[should_panic]
    fn bad_language_code_2t() {
        let x = get("zzz");
        for y in x {
            println!("{}", y);
        }
    }
}
