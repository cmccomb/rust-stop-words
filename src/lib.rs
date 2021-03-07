#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![warn(clippy::missing_docs_in_private_items)]

//! # About
//! Stop words are words that don't carry much meaning, and are typically removed as a preprocessing step before text
//! analysis or natural language processing. This crate contains common stop words for a variety of languages. This crate uses stop word
//! lists from [Stopwords ISO](https://github.com/stopwords-iso) and also from [NLTK](https://www.nltk.org/).

// Strum contains all the trait definitions
mod helpers;
mod language_names;

use helpers::{get_language_from_iso, read_from_bytes};
pub use language_names::{ISO_LANGUAGES, LANGUAGE};

use serde_json::{Result, Value};

/// Let's define a macro to help us out with string matching
#[cfg(feature = "nltk")]
macro_rules! string_match {
    (
        $($language:expr)*,
        $($directory:literal)*,
        [$( $lang:literal ),*]
    ) =>
    {
        match $( $language )? {
            $(
                $(
                    $lang => read_from_bytes(include_bytes!(concat!($directory, "/", $lang))),
                )*
            )*
            _ => panic!(concat!("Unfortunately, the '{}' language is not currently supported. Please make sure that the name of the language is spelled in English."), $( $language )? )
        }
    }
}

/// This function fetches stop words for a language using an enum.
pub fn get(input_language: LANGUAGE) -> Vec<String> {
    // Match the full language name
    let target = ISO_LANGUAGES[input_language as usize];
    get_iso(target)
}

/// This function fetches stop words for a language using a 2-letter ISO code
#[cfg(feature = "nltk")]
pub fn get_iso(input_language: &str) -> Vec<String> {
    let x = get_language_from_iso(input_language);
    match x {
        "ar" => read_from_bytes(include_bytes!("nltk/arabic")),
        "az" => read_from_bytes(include_bytes!("nltk/azerbaijani")),
        "da" => read_from_bytes(include_bytes!("nltk/danish")),
        "nl" => read_from_bytes(include_bytes!("nltk/dutch")),
        "en" => read_from_bytes(include_bytes!("nltk/english")),
        "fi" => read_from_bytes(include_bytes!("nltk/finnish")),
        "fr" => read_from_bytes(include_bytes!("nltk/french")),
        "de" => read_from_bytes(include_bytes!("nltk/german")),
        "el" => read_from_bytes(include_bytes!("nltk/greek")),
        "hu" => read_from_bytes(include_bytes!("nltk/hungarian")),
        "id" => read_from_bytes(include_bytes!("nltk/indonesian")),
        "it" => read_from_bytes(include_bytes!("nltk/italian")),
        "kk" => read_from_bytes(include_bytes!("nltk/kazakh")),
        "ne" => read_from_bytes(include_bytes!("nltk/nepali")),
        "no" => read_from_bytes(include_bytes!("nltk/norwegian")),
        "pt" => read_from_bytes(include_bytes!("nltk/portuguese")),
        "ro" => read_from_bytes(include_bytes!("nltk/romanian")),
        "ru" => read_from_bytes(include_bytes!("nltk/russian")),
        "sl" => read_from_bytes(include_bytes!("nltk/slovenian")),
        "es" => read_from_bytes(include_bytes!("nltk/spanish")),
        "sv" => read_from_bytes(include_bytes!("nltk/swedish")),
        "tg" => read_from_bytes(include_bytes!("nltk/tajik")),
        "tr" => read_from_bytes(include_bytes!("nltk/turkish")),
        _ => panic!(concat!("Unfortunately, the '{}' language is not currently supported. Please make sure that the name of the language is spelled in English."), x )
    }
}

/// This function fetches stop words for a language using a 2-letter ISO code
#[cfg(not(feature = "nltk"))]
pub fn get_iso(input_language: &str) -> Vec<String> {
    let bytes = include_bytes!("iso/stopwords-iso.json");
    let mut json: serde_json::Value = serde_json::from_slice(bytes).unwrap();
    if !json[input_language].is_array() {
        panic!(concat!("Unfortunately, the '{}' language is not currently supported or nonexistent. Please make sure that the name is an appropriate 2-letter ISO code."), input_language )
    }
    let array_of_words = json[input_language].as_array_mut().unwrap();
    array_of_words
        .into_iter()
        .map(|x| x.as_str().unwrap().to_owned())
        .collect()
}
