//!

// Strum contains all the trait definitions
use {std::string::ToString, strum_macros};

/// Enum containing available language names for NLTK, spelled out
#[cfg(feature = "nltk")]
#[non_exhaustive]
#[derive(strum_macros::ToString, Debug, Copy, Clone, PartialEq, strum_macros::EnumString)]
pub enum LANGUAGE {
    /// Arabic
    Arabic = 0,

    ///
    Azerbaijani,

    ///
    Danish,

    ///
    Dutch,

    ///
    English,

    ///
    Finnish,

    ///
    French,

    ///
    German,

    ///
    Greek,

    ///
    Hungarian,

    ///
    Indonesian,

    ///
    Italian,

    ///
    Kazakh,

    ///
    Nepali,

    ///
    Norwegian,

    ///
    Portuguese,

    ///
    Romanian,

    ///
    Russian,

    ///
    Slovenian,

    ///
    Spanish,

    ///
    Swedish,

    ///
    Tajik,

    ///
    Turkish,
}

/// Enum containing available language names for ISO stopwords, spelled out
#[cfg(not(feature = "nltk"))]
#[non_exhaustive]
#[derive(strum_macros::ToString, Debug, Copy, Clone, PartialEq, strum_macros::EnumString)]
pub enum LANGUAGE {
    Afrikaans = 0,
    Arabic,
    Armenian,
    Basque,
    Bengali,
    Breton,
    Bulgarian,
    Catalan,
    Czech,
    Chinese,
    Danish,
    German,
    Dutch,
    Greek,
    English,
    Esperanto,
    Estonian,
    Persian,
    Finnish,
    French,
    Irish,
    Galician,
    Gujarati,
    Hausa,
    Hebrew,
    Hindi,
    Croatian,
    Hungarian,
    Indonesian,
    Italian,
    Japanese,
    Korean,
    Kurdish,
    Latin,
    Latvian,
    Lithuanian,
    Marathi,
    Malay,
    Norwegian,
    Polish,
    Portuguese,
    Romanian,
    Russian,
    Slovak,
    Slovenian,
    Somali,
    Sotho,
    Spanish,
    Swahili,
    Swedish,
    Tagalog,
    Thai,
    Turkish,
    Ukrainian,
    Urdu,
    Vietnamese,
    Yoruba,
    Zulu,
}

/// Constant containing an array of available 2-letter language names for NLTK stopwords
#[cfg(feature = "nltk")]
pub const ISO_LANGUAGES: [&str; 23] = [
    "ar", "az", "da", "nl", "en", "fi", "fr", "de", "el", "hu", "id", "it", "kk", "ne", "no", "pt",
    "ro", "ru", "sl", "es", "sv", "tg", "tr",
];

/// Constant containing an array of available 2-letter language names for ISO Stopwords
#[cfg(not(feature = "nltk"))]
pub const ISO_LANGUAGES: [&str; 58] = [
    "af", "ar", "hy", "eu", "bn", "br", "bg", "ca", "cs", "zh", "da", "de", "nl", "el", "en", "eo",
    "et", "fa", "fi", "fr", "ga", "gl", "gu", "ha", "he", "hi", "hr", "hu", "id", "it", "ja", "ko",
    "ku", "la", "lv", "lt", "mr", "ms", "no", "pl", "pt", "ro", "ru", "sk", "sl", "so", "st", "es",
    "sw", "sv", "tl", "th", "tr", "uk", "ur", "vi", "yo", "zu",
];
