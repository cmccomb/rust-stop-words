#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![warn(clippy::missing_docs_in_private_items)]

//! Helper functions

// Strum contains all the trait definitions
use super::language_names::{ISO_LANGUAGES, LANGUAGE};

/// This function converts a language code to a full language name
pub(crate) fn get_language_from_iso(code: &str) -> &str {
    let mut iter = ISO_LANGUAGES.iter();
    let idx = iter.position(|&x| x == code);
    match idx {
        Some(x) => ISO_LANGUAGES[x],
        None => panic!(
            "The {} language code is nonexistent or not currently supported.",
            code
        ),
    }
}

/// This function converts the bytestring to a vector
pub(crate) fn read_from_bytes(bytes: &[u8]) -> Vec<String> {
    let contents = String::from_utf8_lossy(bytes);
    let split_contents = contents.split('\n');
    let mut output = vec![];
    for word in split_contents {
        output.push(String::from(word));
    }
    output
}
