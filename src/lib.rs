#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]

mod language_names;
pub use language_names::LANGUAGE;

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
    get_names_from_iso_code(input_language.into())
}

/// This function fetches stop words for a language using a 2-letter ISO code
#[cfg(feature = "nltk")]
fn get_names_from_iso_code(input_language: String) -> Vec<String> {
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

    // This matches against the language
    match &*input_language {
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
        _ => panic!("The '{}' language is not recognized. Please check the documentation for a supported list of languages.", input_language)
    }
}

/// This function fetches stop words for a language using a 2-letter ISO code
#[cfg(not(feature = "nltk"))]
fn get_names_from_iso_code(input_language: String) -> Vec<String> {
    let bytes = include_bytes!("iso/stopwords-iso.json");
    let mut json: serde_json::Value = serde_json::from_slice(bytes).unwrap();
    if !json[&*input_language].is_array() {
        panic!(concat!("Unfortunately, the '{}' language is not currently supported or nonexistent. Please make sure that the name is an appropriate 2-letter ISO code."), input_language )
    }
    let array_of_words = json[&*input_language].as_array_mut().unwrap();
    array_of_words
        .into_iter()
        .map(|x| x.as_str().unwrap().to_owned())
        .collect()
}
