#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! This module contains stop words for a number of languages, using data from [this resource](https://github.com/Alir3z4/stop-words/tree/bd8cc1434faeb3449735ed570a4a392ab5d35291)
//! ```
//! use stop_words;
//! let x = stop_words::get("english");
//!
//! ```

/// Constant containing an array of available language names
pub const LANGUAGES: [&str; 25] = ["arabic", "catalan", "danish", "english", "french", "hindi",
    "indonesian", "norwegian", "portuguese", "russian", "spanish", "turkish", "vietnamese",
    "bulgarian", "czech", "dutch", "finnish", "german", "hungarian", "italian", "polish",
    "romanian", "slovak", "swedish", "ukrainian"];

// TODO: Add support for ISO language codes
/// The only function you'll ever need! Given a language code it returns common stop words
pub fn get(language: &str) -> Vec<String> {
    match language {
        "english" => read_from_bytes(include_bytes!("english.txt")),
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
        _ => panic!("The {} language is not currently supported.", language)
    }
}

/// This function checks if a language is available
pub fn language_available(language: &str) -> bool {
    let mut language_available = false;
    for language_option in LANGUAGES.iter() {
        if language == *language_option {
            language_available = true;
            break;
        }
    }
    language_available
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
    #[should_panic]
    fn bad_language_name() {
        let x = get("engsh");
        for y in x {
            println!("{}", y);
        }
    }
}
