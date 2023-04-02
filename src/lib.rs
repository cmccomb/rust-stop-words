#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]

mod language_names;
pub use language_names::LANGUAGE;

/// This function fetches stop words for a language using either a member of the `LANGUAGE` enum,
/// or a two-character ISO language name as either a `str` or a `String` type.
/// ```
/// let first_list = stop_words::get("ar");
/// let second_list = stop_words::get(stop_words::LANGUAGE::Arabic);
/// assert_eq!(first_list, second_list)
/// ```
pub fn get<T: Into<String>>(input_language: T) -> Vec<String> {
    // Check the input
    let language_name_as_string = input_language.into();

    // Get the bytes
    let json_as_bytes: &[u8] = if cfg!(feature = "nltk") {
        include_bytes!(concat!(env!("OUT_DIR"), "/nltk_file.json"))
    } else {
        include_bytes!("iso/stopwords-iso.json")
    };

    // Get the JSON
    let json: serde_json::Value = serde_json::from_slice(json_as_bytes)
        .expect("Could not read JSON file from Stopwords ISO.");

    // Get the words
    json.get(language_name_as_string.clone())
        .unwrap_or_else(|| panic!("The '{language_name_as_string}' language is not recognized. Please check the documentation for a supported list of languages."))
        .clone()
        .as_array_mut()
        .expect("The referenced value is not a mutable array.")
        .iter()
        .map(serde_json::Value::to_string)
        .collect()
}
