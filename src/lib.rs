#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]

mod language_names;
pub use language_names::LANGUAGE;
use serde_json::Value;

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
    // Check the input
    let language_name_as_enum = LANGUAGE::from(input_language.into());

    // Get the bytes
    let json_as_bytes: &[u8] = if cfg!(feature = "nltk") {
        include_bytes!(concat!(env!("OUT_DIR"), "/nltk_file.json"))
    } else {
        include_bytes!("iso/stopwords-iso.json")
    };

    // Get the JSON
    let mut json: Value = serde_json::from_slice(json_as_bytes)
        .expect("Could not read JSON file from Stopwords ISO.");

    // Get the words
    json[String::from(language_name_as_enum)]
        .as_array_mut()
        .expect("The '{input_language}' language is not recognized. Please check the documentation for a supported list of languages.")
        .iter()
        .map(Value::to_string)
        .collect()
}
