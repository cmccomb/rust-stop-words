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

/// This function fetches stop words for a language using either a member of the `LANGUAGE` enum,
/// or a two-character ISO language code as any type implementing [`std::convert::AsRef`]`<str>`.
/// Please note that constructed languages use either a member of the `LANGUAGE` enum, or a
/// __three__-character ISO language code as either a `str` or a `String` type.
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
