#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]

mod language_names;
pub use crate::language_names::Language;
#[allow(deprecated)]
pub use crate::language_names::LANGUAGE;

#[allow(clippy::missing_docs_in_private_items)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/stopwords.rs"));
}

/// Returns the language codes enabled by this crate's active feature set.
///
/// The returned slice is sorted lexicographically and contains ISO 639 codes
/// plus any enabled project-specific identifiers such as `"hinglish"`.
#[must_use]
pub const fn available_languages() -> &'static [&'static str] {
    generated::AVAILABLE_LANGUAGES
}

/// Looks up stop words without panicking for an unsupported or disabled language.
///
/// Accepts a [`Language`], a language code, or any other type implementing
/// [`AsRef<str>`]. Codes are matched exactly; no case or whitespace normalization
/// is performed. Returns `None` when a code is unknown or its data is disabled.
/// This is the non-panicking counterpart to [`get`].
///
/// ```
/// let code = String::from("en");
/// assert_eq!(stop_words::lookup(&code), stop_words::lookup("en"));
/// assert!(stop_words::lookup("not-a-language").is_none());
/// ```
#[must_use]
pub fn lookup(language: impl AsRef<str>) -> Option<&'static [&'static str]> {
    generated::lookup(language.as_ref())
}

/// This function fetches stop words for a language using either a member of the [`Language`] enum
/// or a lookup code as any type implementing [`std::convert::AsRef`]`<str>`.
/// For most languages the lookup code is a two-character ISO 639-1 code.
/// Constructed languages use three-character codes (`dot`, `dov`, `nav`, `qya`, `sjn`, `tlh`,
/// `val`), and NLTK-specific `hinglish` is supported when the `nltk` feature is enabled.
/// Prefer [`lookup`] when the language is supplied at runtime or may be disabled.
///
/// ```
/// if let Some(words) = stop_words::lookup("en") {
///     assert_eq!(stop_words::get("en"), words);
/// }
/// ```
/// # Panics
///
/// Panics if the provided language code is unknown or its data is disabled.
pub fn get<T: std::convert::AsRef<str>>(input_language: T) -> &'static [&'static str] {
    let language_name: &str = input_language.as_ref();
    lookup(language_name).unwrap_or_else(|| unknown_language(language_name))
}

#[cold]
#[track_caller]
/// Panics with the shared diagnostic for infallible lookup functions.
fn unknown_language(language_name: &str) -> ! {
    panic!(
        "The '{language_name}' language is not recognized. Please check `available_languages()` for the codes enabled by this crate's active features."
    )
}
