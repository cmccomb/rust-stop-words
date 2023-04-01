#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]

mod language_names;
pub use language_names::LANGUAGE;
use serde_json::Value;

#[cfg(any(feature = "iso", feature = "nltk"))]
/// This function  fetches stop words for a language using either a member of the `LANGUAGE` enum,
/// or a two-character ISO language name as either a `str` or a `String` type.
/// ```
/// let first_list = stop_words::get("ar");
/// let second_list = stop_words::get(stop_words::LANGUAGE::Arabic);
/// assert_eq!(first_list, second_list)
/// ```
pub fn get<T>(input_language: T) -> Vec<String>
where
    T: Into<String>,
{
    get_words_from_iso_code(LANGUAGE::from(input_language.into()))
}

/// This function fetches stop words for a language using a 2-letter ISO code
#[cfg(feature = "nltk")]
fn get_words_from_iso_code(input_language: LANGUAGE) -> Vec<String> {
    // This matches against the language
    let bytes: &[u8] = match input_language {
        LANGUAGE::Arabic => include_bytes!("nltk/arabic"),
        LANGUAGE::Azerbaijani => include_bytes!("nltk/azerbaijani"),
        LANGUAGE::Danish => include_bytes!("nltk/danish"),
        LANGUAGE::Dutch => include_bytes!("nltk/dutch"),
        LANGUAGE::English => include_bytes!("nltk/english"),
        LANGUAGE::Finnish => include_bytes!("nltk/finnish"),
        LANGUAGE::French => include_bytes!("nltk/french"),
        LANGUAGE::German => include_bytes!("nltk/german"),
        LANGUAGE::Greek => include_bytes!("nltk/greek"),
        LANGUAGE::Hungarian => include_bytes!("nltk/hungarian"),
        LANGUAGE::Indonesian => include_bytes!("nltk/indonesian"),
        LANGUAGE::Italian => include_bytes!("nltk/italian"),
        LANGUAGE::Kazakh => include_bytes!("nltk/kazakh"),
        LANGUAGE::Nepali => include_bytes!("nltk/nepali"),
        LANGUAGE::Norwegian => include_bytes!("nltk/norwegian"),
        LANGUAGE::Portuguese => include_bytes!("nltk/portuguese"),
        LANGUAGE::Romanian => include_bytes!("nltk/romanian"),
        LANGUAGE::Russian => include_bytes!("nltk/russian"),
        LANGUAGE::Slovenian => include_bytes!("nltk/slovenian"),
        LANGUAGE::Spanish => include_bytes!("nltk/spanish"),
        LANGUAGE::Swedish => include_bytes!("nltk/swedish"),
        LANGUAGE::Tajik => include_bytes!("nltk/tajik"),
        LANGUAGE::Turkish => include_bytes!("nltk/turkish"),
        _ => {
            // The `get` function checks to ensure that the language code is valid.
            unreachable!()
        }
    };

    // Make the words!
    String::from_utf8_lossy(bytes)
        .split('\n')
        .map(String::from)
        .collect()
}

/// This function fetches stop words for a language using a 2-letter ISO code
#[cfg(not(feature = "nltk"))]
fn get_words_from_iso_code(input_language: LANGUAGE) -> Vec<String> {
    // Compile JSON file into the `bytes` variable and deserialize with serde
    let json_as_bytes = include_bytes!("iso/stopwords-iso.json");
    let mut json: Value = serde_json::from_slice(json_as_bytes)
        .expect("Could not read  JSON file from Stopwords ISO.");

    // Make the words!
    json[String::from(input_language)]
        .as_array_mut()
        .expect("Cannot extract a mutable array from JSON file.")
        .iter()
        .map(serde_json::Value::to_string)
        .collect()
}
