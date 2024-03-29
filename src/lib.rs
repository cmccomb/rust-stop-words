#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]

mod language_names;
pub use language_names::LANGUAGE;

/// This function fetches stop words for a language using either a member of the `LANGUAGE` enum,
/// or a two-character ISO language code as either a `str` or a `String` type. Please note that
/// constructed languages use either a member of the `LANGUAGE` enum, or a __three__-character ISO
/// language code as either a `str` or a `String` type
/// ```ignore
/// let first_list = stop_words::get("ar");
/// let second_list = stop_words::get(stop_words::LANGUAGE::Arabic);
/// assert_eq!(first_list, second_list)
/// ```
pub fn get<T: Into<String>>(input_language: T) -> Vec<String> {
    // Check the input
    let language_name_as_string = input_language.into();

    // Get the bytes
    let json_as_bytes: &[u8] = if cfg!(feature = "nltk") {
        include_bytes!(concat!(env!("OUT_DIR"), "/stopwords-nltk.json"))
    } else if cfg!(feature = "constructed") {
        include_bytes!(concat!(env!("OUT_DIR"), "/stopwords-constructed.json"))
    } else {
        include_bytes!("iso/stopwords-iso.json")
    };

    // Get the JSON
    let mut json: serde_json::Value = serde_json::from_slice(json_as_bytes)
        .expect("Could not read JSON file from Stopwords ISO.");

    // Get the words
    json.get_mut(&language_name_as_string)
        .take()
        .unwrap_or_else(|| panic!("The '{language_name_as_string}' language is not recognized. Please check the documentation for a supported list of languages."))
        .as_array_mut()
        .expect("The referenced value is not a mutable array.")
        .iter_mut()
        .map(|x| {
            let x = x.take();
            if let serde_json::Value::String(s) = x {
                s
            } else {
                panic!("The referenced value is not a string.")
            }
        })
        .collect()
}
