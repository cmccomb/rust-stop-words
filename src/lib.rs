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

// Strum contains all the trait definitions
#[cfg(feature = "enum")]
use {std::string::ToString, strum_macros};

/// Enum containing available language names, spelled out
#[cfg(feature = "enum")]
#[non_exhaustive]
#[derive(strum_macros::ToString, Debug, Copy, Clone)]
pub enum LANGUAGE {
    Arabic,
    Azerbaijani,
    Catalan,
    Danish,
    English,
    French,
    Hindi,
    Indonesian,
    Norwegian,
    Portuguese,
    Russian,
    Spanish,
    Turkish,
    Vietnamese,
    Bulgarian,
    Czech,
    Dutch,
    Finnish,
    German,
    Hungarian,
    Italian,
    Polish,
    Romanian,
    Slovak,
    Swedish,
    Ukrainian,
    Hebrew,
    Greek,
    Kazakh,
    Nepali,
    Slovenian,
    Tajik,
}

/// Constant containing an array of available language names, spelled out
pub const LANGUAGES: [&str; 32] = [
    "arabic",
    "azerbaijani",
    "catalan",
    "danish",
    "english",
    "french",
    "hindi",
    "indonesian",
    "norwegian",
    "portuguese",
    "russian",
    "spanish",
    "turkish",
    "vietnamese",
    "bulgarian",
    "czech",
    "dutch",
    "finnish",
    "german",
    "hungarian",
    "italian",
    "polish",
    "romanian",
    "slovak",
    "swedish",
    "ukrainian",
    "hebrew",
    "greek",
    "kazakh",
    "nepali",
    "slovenian",
    "tajik",
];

/// Constant containing an array of available language names, using ISO-693-1 codes
pub const LANGUAGES_ISO_693_1: [&str; 32] = [
    "ar", "az", "ca", "da", "en", "fr", "hi", "in", "nn", "pt", "ru", "es", "tr", "vi", "bg", "cs",
    "nl", "fi", "de", "hu", "it", "pl", "ro", "sk", "sv", "uk", "he", "el", "kk", "ne", "sl", "tg",
];

/// Constant containing an array of available language names, using ISO-693-2T codes
pub const LANGUAGES_ISO_693_2T: [&str; 32] = [
    "ara", "aze", "cat", "dan", "eng", "fra", "hin", "ind", "nno", "por", "rus", "spa", "tur",
    "vie", "bul", "ces", "nld", "fin", "deu", "hun", "ita", "pol", "ron", "slk", "swe", "ukr",
    "heb", "ell", "kaz", "nep", "slv", "tgk",
];

/// Let's define a macro to help us out with string matching
macro_rules! string_match {
    (
        $($language:expr)*,
        $(
            $directory:literal, [$( $lang:literal ),*]
        ),*
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

/// The only function you'll ever need! Given a language code or name it returns common stop words as a ``Vec<String>``
///
/// ```
/// #[cfg(not(feature = "enum"))]
/// let vec = stop_words::get("spanish");
/// #[cfg(feature = "enum")]
/// let vec = stop_words::get(stop_words::LANGUAGE::Spanish);
/// ```
pub fn get(
    #[cfg(feature = "enum")] input_language: LANGUAGE,
    #[cfg(not(feature = "enum"))] input_language: &'static str,
) -> Vec<String> {
    string_match!(
        parse(input_language),
        "savand",
        [
            "english",
            "hebrew",
            "arabic",
            "catalan",
            "danish",
            "french",
            "hindi",
            "indonesian",
            "norwegian",
            "portuguese",
            "russian",
            "spanish",
            "turkish",
            "vietnamese",
            "bulgarian",
            "czech",
            "dutch",
            "finnish",
            "german",
            "hungarian",
            "italian",
            "polish",
            "romanian",
            "slovak",
            "swedish",
            "ukrainian"
        ],
        "nltk",
        [
            "azerbaijani",
            "greek",
            "kazakh",
            "nepali",
            "slovenian",
            "tajik"
        ]
    )
}

/// Ok, you might need this function too. It fetches stop words specifically for NLTK.
///
/// ```
/// #[cfg(not(feature = "enum"))]
/// let vec = stop_words::get_nltk("spanish");
/// #[cfg(feature = "enum")]
/// let vec = stop_words::get_nltk(stop_words::LANGUAGE::Spanish);
/// ```
pub fn get_nltk(
    #[cfg(feature = "enum")] input_language: LANGUAGE,
    #[cfg(not(feature = "enum"))] input_language: &'static str,
) -> Vec<String> {
    // Match the full language name
    string_match!(
        parse(input_language),
        "nltk",
        [
            "english",
            "arabic",
            "danish",
            "french",
            "indonesian",
            "norwegian",
            "portuguese",
            "russian",
            "spanish",
            "turkish",
            "greek",
            "dutch",
            "finnish",
            "german",
            "hungarian",
            "italian",
            "romanian",
            "swedish",
            "azerbaijani",
            "kazakh",
            "nepali",
            "slovenian",
            "tajik"
        ]
    )
}

/// This is a helper function to resolve inputs when using different features
fn parse(
    #[cfg(feature = "enum")] input_language: LANGUAGE,
    #[cfg(not(feature = "enum"))] input_language: &'static str,
) -> &'static str {
    #[cfg(feature = "enum")]
    let target_string: &str = Box::leak(input_language.to_string().to_lowercase().into_boxed_str());
    #[cfg(not(feature = "enum"))]
    let target_string: &str = get_language_from_code(input_language);
    return target_string;
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
        None => panic!("It looks like you're trying to use an ISO 693 language code. Unfortunately, the {} language code is not currently supported.", code),
    }
}

/// This function converts the bytestring to a vector
fn read_from_bytes(bytes: &[u8]) -> Vec<String> {
    let contents = String::from_utf8_lossy(bytes);
    let split_contents = contents.split('\n');
    let mut output = vec![];
    for word in split_contents {
        output.push(String::from(word));
    }
    output
}
