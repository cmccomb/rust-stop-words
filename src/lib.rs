#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]

mod language_names;
pub use crate::language_names::LANGUAGE;

#[allow(clippy::missing_docs_in_private_items)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/stopwords.rs"));
}

use crate::generated::lookup;

/// This function fetches stop words for a language using either a member of the `LANGUAGE` enum
/// or a lookup code as any type implementing [`std::convert::AsRef`]`<str>`.
/// For most languages the lookup code is a two-character ISO 639-1 code.
/// Constructed languages use three-character codes (`dot`, `dov`, `nav`, `qya`, `sjn`, `tlh`,
/// `val`), and NLTK-specific `hinglish` is supported when the `nltk` feature is enabled.
/// ```ignore
/// let first_list = stop_words::get("ar");
/// let second_list = stop_words::get(stop_words::LANGUAGE::Arabic);
/// assert_eq!(first_list, second_list)
/// ```
/// # Panics
///
/// Panics if the provided language code is not recognized.
pub fn get<T: std::convert::AsRef<str>>(input_language: T) -> &'static [&'static str] {
    let language_name: &str = input_language.as_ref();
    lookup(language_name).unwrap_or_else(|| {
        panic!(
            "The '{language_name}' language is not recognized. Please check the documentation for a supported list of languages."
        )
    })
}
