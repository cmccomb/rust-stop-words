#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]

mod language_names;
pub use language_names::LANGUAGE;

include!(concat!(env!("OUT_DIR"), "/stopwords.rs"));

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
    let language_name_as_string = input_language.into();
    stop_words(language_name_as_string.as_str())
        .unwrap_or_else(|| panic!("The '{language_name_as_string}' language is not recognized. Please check the documentation for a supported list of languages."))
        .iter()
        .map(|word| std::string::ToString::to_string(*word))
        .collect()
}
