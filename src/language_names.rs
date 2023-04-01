//! Module containing the huge language enum and formatting for it

use std::fmt::Display;
use std::fmt::Formatter;

/// Enum containing available language names
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum LANGUAGE {
    #[cfg(any(feature = "iso", feature = "nltk"))]
    /// Arabic (ISO 639-1 Code: ar)
    Arabic,

    #[cfg(feature = "nltk")]
    /// Azerbaijani (ISO 639-1 Code: az)
    Azerbaijani,

    /// Danish (ISO 639-1 Code: da)
    Danish,

    #[cfg(any(feature = "iso", feature = "nltk"))]
    /// Dutch (ISO 639-1 Code: nl)
    Dutch,

    #[cfg(any(feature = "iso", feature = "nltk"))]
    /// English (ISO 639-1 Code: en)
    English,

    #[cfg(any(feature = "iso", feature = "nltk"))]
    /// Finnish (ISO 639-1 Code: fi)
    Finnish,

    #[cfg(any(feature = "iso", feature = "nltk"))]
    /// French (ISO 639-1 Code: fr)
    French,

    #[cfg(any(feature = "iso", feature = "nltk"))]
    /// German (ISO 639-1 Code: de)
    German,

    #[cfg(any(feature = "iso", feature = "nltk"))]
    /// Greek (ISO 639-1 Code: el)
    Greek,

    #[cfg(any(feature = "iso", feature = "nltk"))]
    /// Hungarian (ISO 639-1 Code: hu)
    Hungarian,

    #[cfg(any(feature = "iso", feature = "nltk"))]
    /// Indonesian (ISO 639-1 Code: id)
    Indonesian,

    #[cfg(any(feature = "iso", feature = "nltk"))]
    /// Italian (ISO 639-1 Code: it)
    Italian,

    #[cfg(feature = "nltk")]
    /// Kazakh (ISO 639-1 Code: kk)
    Kazakh,

    #[cfg(feature = "nltk")]
    /// Nepali (ISO 639-1 Code: ne)
    Nepali,

    #[cfg(any(feature = "iso", feature = "nltk"))]
    /// Norwegian (ISO 639-1 Code: no)
    Norwegian,

    #[cfg(any(feature = "iso", feature = "nltk"))]
    /// Portuguese (ISO 639-1 Code: pt)
    Portuguese,

    #[cfg(any(feature = "iso", feature = "nltk"))]
    /// Romanian (ISO 639-1 Code: ro)
    Romanian,

    #[cfg(any(feature = "iso", feature = "nltk"))]
    /// Russian (ISO 639-1 Code: ru)
    Russian,

    #[cfg(any(feature = "iso", feature = "nltk"))]
    /// Slovenian (ISO 639-1 Code: sl)
    Slovenian,

    #[cfg(any(feature = "iso", feature = "nltk"))]
    /// Spanish (ISO 639-1 Code: sp)
    Spanish,

    #[cfg(any(feature = "iso", feature = "nltk"))]
    /// Swedish (ISO 639-1 Code: sv)
    Swedish,

    #[cfg(feature = "nltk")]
    /// Tajik (ISO 639-1 Code: tg)
    Tajik,

    #[cfg(any(feature = "iso", feature = "nltk"))]
    /// Turkish (ISO 639-1 Code: tk)
    Turkish,

    #[cfg(feature = "iso")]
    /// Afrikaans (ISO 639-1 Code: af)
    Afrikaans,

    #[cfg(feature = "iso")]
    /// Armenian (ISO 639-1 Code: hy)
    Armenian,

    #[cfg(feature = "iso")]
    /// Basque (ISO 639-1 Code: eu)
    Basque,

    #[cfg(feature = "iso")]
    /// Bengali (ISO 639-1 Code: bn)
    Bengali,

    #[cfg(feature = "iso")]
    /// Breton (ISO 639-1 Code: br)
    Breton,

    #[cfg(feature = "iso")]
    /// Bulgarian (ISO 639-1 Code: bg)
    Bulgarian,

    #[cfg(feature = "iso")]
    /// Catalan (ISO 639-1 Code: ca)
    Catalan,

    #[cfg(feature = "iso")]
    /// Czech (ISO 639-1 Code: cs)
    Czech,

    #[cfg(feature = "iso")]
    /// Chinese (ISO 639-1 Code: zh)
    Chinese,

    #[cfg(feature = "iso")]
    /// Esperanto (ISO 639-1 Code: eo)
    Esperanto,

    #[cfg(feature = "iso")]
    /// Estonian (ISO 639-1 Code: eo)
    Estonian,

    #[cfg(feature = "iso")]
    /// Persian (ISO 639-1 Code: fa)
    Persian,

    #[cfg(feature = "iso")]
    /// Irish (ISO 639-1 Code: ga)
    Irish,

    #[cfg(feature = "iso")]
    /// Galician (ISO 639-1 Code: gl)
    Galician,

    #[cfg(feature = "iso")]
    /// Gujarati (ISO 639-1 Code: gu)
    Gujarati,

    #[cfg(feature = "iso")]
    /// Hausa (ISO 639-1 Code: ha)
    Hausa,

    #[cfg(feature = "iso")]
    /// Hebrew (ISO 639-1 Code: he)
    Hebrew,

    #[cfg(feature = "iso")]
    /// Hindi (ISO 639-1 Code: hi)
    Hindi,

    #[cfg(feature = "iso")]
    /// Croatian (ISO 639-1 Code: hr)
    Croatian,

    #[cfg(feature = "iso")]
    /// Japanese (ISO 639-1 Code: ha)
    Japanese,

    #[cfg(feature = "iso")]
    /// Korean (ISO 639-1 Code: ko)
    Korean,

    #[cfg(feature = "iso")]
    /// Kurdish (ISO 639-1 Code: ku)
    Kurdish,

    #[cfg(feature = "iso")]
    /// Latin (ISO 639-1 Code: la)
    Latin,

    #[cfg(feature = "iso")]
    /// Latvian (ISO 639-1 Code: lv)
    Latvian,

    #[cfg(feature = "iso")]
    /// Lithuanian (ISO 639-1 Code: lt)
    Lithuanian,

    #[cfg(feature = "iso")]
    /// Marathi (ISO 639-1 Code: mr)
    Marathi,

    #[cfg(feature = "iso")]
    /// Malay (ISO 639-1 Code: ms)
    Malay,

    #[cfg(feature = "iso")]
    /// Polish (ISO 639-1 Code: pl)
    Polish,

    #[cfg(feature = "iso")]
    /// Slovak (ISO 639-1 Code: sk)
    Slovak,

    #[cfg(feature = "iso")]
    /// Somali (ISO 639-1 Code: so)
    Somali,

    #[cfg(feature = "iso")]
    /// Sotho (ISO 639-1 Code: st)
    Sotho,

    #[cfg(feature = "iso")]
    /// Swahili (ISO 639-1 Code: sw)
    Swahili,

    #[cfg(feature = "iso")]
    /// Taglog (ISO 639-1 Code: tl)
    Tagalog,

    #[cfg(feature = "iso")]
    /// Thai (ISO 639-1 Code: th)
    Thai,

    #[cfg(feature = "iso")]
    /// Ukrainian (ISO 639-1 Code: uk)
    Ukrainian,

    #[cfg(feature = "iso")]
    /// Urdu (ISO 639-1 Code: ur)
    Urdu,

    #[cfg(feature = "iso")]
    /// Vietnamese (ISO 639-1 Code: vi)
    Vietnamese,

    #[cfg(feature = "iso")]
    /// Yoruba (ISO 639-1 Code: yo)
    Yoruba,

    #[cfg(feature = "iso")]
    /// Zulu (ISO 639-1 Code: zu)
    Zulu,

    #[cfg(feature = "unimplemented")]
    /// Afar (ISO 639-1 Code: af)
    Afar,
}

impl From<String> for LANGUAGE {
    fn from(value: String) -> Self {
        match value.as_str() {
            #[cfg(any(feature = "iso", feature = "nltk"))]
            "ar" => LANGUAGE::Arabic,
            #[cfg(feature = "nltk")]
            "az" => LANGUAGE::Azerbaijani,
            #[cfg(any(feature = "iso", feature = "nltk"))]
            "da" => LANGUAGE::Danish,
            #[cfg(any(feature = "iso", feature = "nltk"))]
            "nl" => LANGUAGE::Dutch,
            #[cfg(any(feature = "iso", feature = "nltk"))]
            "en" => LANGUAGE::English,
            #[cfg(any(feature = "iso", feature = "nltk"))]
            "fi" => LANGUAGE::Finnish,
            #[cfg(any(feature = "iso", feature = "nltk"))]
            "fr" => LANGUAGE::French,
            #[cfg(any(feature = "iso", feature = "nltk"))]
            "de" => LANGUAGE::German,
            #[cfg(any(feature = "iso", feature = "nltk"))]
            "el" => LANGUAGE::Greek,
            #[cfg(any(feature = "iso", feature = "nltk"))]
            "hu" => LANGUAGE::Hungarian,
            #[cfg(any(feature = "iso", feature = "nltk"))]
            "id" => LANGUAGE::Indonesian,
            #[cfg(any(feature = "iso", feature = "nltk"))]
            "it" => LANGUAGE::Italian,
            #[cfg(feature = "nltk")]
            "kk" => LANGUAGE::Kazakh,
            #[cfg(feature = "nltk")]
            "ne" => LANGUAGE::Nepali,
            #[cfg(any(feature = "iso", feature = "nltk"))]
            "no" => LANGUAGE::Norwegian,
            #[cfg(any(feature = "iso", feature = "nltk"))]
            "pt" => LANGUAGE::Portuguese,
            #[cfg(any(feature = "iso", feature = "nltk"))]
            "ro" => LANGUAGE::Romanian,
            #[cfg(any(feature = "iso", feature = "nltk"))]
            "ru" => LANGUAGE::Russian,
            #[cfg(any(feature = "iso", feature = "nltk"))]
            "sl" => LANGUAGE::Slovenian,
            #[cfg(any(feature = "iso", feature = "nltk"))]
            "es" => LANGUAGE::Spanish,
            #[cfg(any(feature = "iso", feature = "nltk"))]
            "sv" => LANGUAGE::Swedish,
            #[cfg(feature = "nltk")]
            "tg" => LANGUAGE::Tajik,
            #[cfg(any(feature = "iso", feature = "nltk"))]
            "tr" => LANGUAGE::Turkish,
            #[cfg(feature = "iso")]
            "af" => LANGUAGE::Afrikaans,
            #[cfg(feature = "iso")]
            "hy" => LANGUAGE::Armenian,
            #[cfg(feature = "iso")]
            "eu" => LANGUAGE::Basque,
            #[cfg(feature = "iso")]
            "bn" => LANGUAGE::Bengali,
            #[cfg(feature = "iso")]
            "br" => LANGUAGE::Breton,
            #[cfg(feature = "iso")]
            "bg" => LANGUAGE::Bulgarian,
            #[cfg(feature = "iso")]
            "ca" => LANGUAGE::Catalan,
            #[cfg(feature = "iso")]
            "cs" => LANGUAGE::Czech,
            #[cfg(feature = "iso")]
            "zh" => LANGUAGE::Chinese,
            #[cfg(feature = "iso")]
            "eo" => LANGUAGE::Esperanto,
            #[cfg(feature = "iso")]
            "et" => LANGUAGE::Estonian,
            #[cfg(feature = "iso")]
            "fa" => LANGUAGE::Persian,
            #[cfg(feature = "iso")]
            "ga" => LANGUAGE::Irish,
            #[cfg(feature = "iso")]
            "gl" => LANGUAGE::Galician,
            #[cfg(feature = "iso")]
            "gu" => LANGUAGE::Gujarati,
            #[cfg(feature = "iso")]
            "ha" => LANGUAGE::Hausa,
            #[cfg(feature = "iso")]
            "he" => LANGUAGE::Hebrew,
            #[cfg(feature = "iso")]
            "hi" => LANGUAGE::Hindi,
            #[cfg(feature = "iso")]
            "hr" => LANGUAGE::Croatian,
            #[cfg(feature = "iso")]
            "ja" => LANGUAGE::Japanese,
            #[cfg(feature = "iso")]
            "ko" => LANGUAGE::Korean,
            #[cfg(feature = "iso")]
            "ku" => LANGUAGE::Kurdish,
            #[cfg(feature = "iso")]
            "la" => LANGUAGE::Latin,
            #[cfg(feature = "iso")]
            "lv" => LANGUAGE::Latvian,
            #[cfg(feature = "iso")]
            "lt" => LANGUAGE::Lithuanian,
            #[cfg(feature = "iso")]
            "mr" => LANGUAGE::Marathi,
            #[cfg(feature = "iso")]
            "ms" => LANGUAGE::Malay,
            #[cfg(feature = "iso")]
            "pl" => LANGUAGE::Polish,
            #[cfg(feature = "iso")]
            "sk" => LANGUAGE::Slovak,
            #[cfg(feature = "iso")]
            "so" => LANGUAGE::Somali,
            #[cfg(feature = "iso")]
            "st" => LANGUAGE::Sotho,
            #[cfg(feature = "iso")]
            "sw" => LANGUAGE::Swahili,
            #[cfg(feature = "iso")]
            "tl" => LANGUAGE::Tagalog,
            #[cfg(feature = "iso")]
            "th" => LANGUAGE::Thai,
            #[cfg(feature = "iso")]
            "uk" => LANGUAGE::Ukrainian,
            #[cfg(feature = "iso")]
            "ur" => LANGUAGE::Urdu,
            #[cfg(feature = "iso")]
            "vi" => LANGUAGE::Vietnamese,
            #[cfg(feature = "iso")]
            "yo" => LANGUAGE::Yoruba,
            #[cfg(feature = "iso")]
            "zu" => LANGUAGE::Zulu,
            #[cfg(feature = "unimplemented")]
            "af" => LANGUAGE::Afar,
            _ => {
                panic!(
                    "The '{}' language is not recognized. Please check the documentation for a supported list of languages.",
                    value
                )
            }
        }
    }
}

impl From<LANGUAGE> for String {
    fn from(value: LANGUAGE) -> Self {
        match value {
            #[cfg(any(feature = "iso", feature = "nltk"))]
            LANGUAGE::Arabic => "ar",
            #[cfg(feature = "nltk")]
            LANGUAGE::Azerbaijani => "az",
            #[cfg(any(feature = "iso", feature = "nltk"))]
            LANGUAGE::Danish => "da",
            #[cfg(any(feature = "iso", feature = "nltk"))]
            LANGUAGE::Dutch => "nl",
            #[cfg(any(feature = "iso", feature = "nltk"))]
            LANGUAGE::English => "en",
            #[cfg(any(feature = "iso", feature = "nltk"))]
            LANGUAGE::Finnish => "fi",
            #[cfg(any(feature = "iso", feature = "nltk"))]
            LANGUAGE::French => "fr",
            #[cfg(any(feature = "iso", feature = "nltk"))]
            LANGUAGE::German => "de",
            #[cfg(any(feature = "iso", feature = "nltk"))]
            LANGUAGE::Greek => "el",
            #[cfg(any(feature = "iso", feature = "nltk"))]
            LANGUAGE::Hungarian => "hu",
            #[cfg(any(feature = "iso", feature = "nltk"))]
            LANGUAGE::Indonesian => "id",
            #[cfg(any(feature = "iso", feature = "nltk"))]
            LANGUAGE::Italian => "it",
            #[cfg(feature = "nltk")]
            LANGUAGE::Kazakh => "kk",
            #[cfg(feature = "nltk")]
            LANGUAGE::Nepali => "ne",
            #[cfg(any(feature = "iso", feature = "nltk"))]
            LANGUAGE::Norwegian => "no",
            #[cfg(any(feature = "iso", feature = "nltk"))]
            LANGUAGE::Portuguese => "pt",
            #[cfg(any(feature = "iso", feature = "nltk"))]
            LANGUAGE::Romanian => "ro",
            #[cfg(any(feature = "iso", feature = "nltk"))]
            LANGUAGE::Russian => "ru",
            #[cfg(any(feature = "iso", feature = "nltk"))]
            LANGUAGE::Slovenian => "sl",
            #[cfg(any(feature = "iso", feature = "nltk"))]
            LANGUAGE::Spanish => "es",
            #[cfg(any(feature = "iso", feature = "nltk"))]
            LANGUAGE::Swedish => "sv",
            #[cfg(feature = "nltk")]
            LANGUAGE::Tajik => "tg",
            #[cfg(any(feature = "iso", feature = "nltk"))]
            LANGUAGE::Turkish => "tr",
            #[cfg(feature = "iso")]
            LANGUAGE::Afrikaans => "af",
            #[cfg(feature = "iso")]
            LANGUAGE::Armenian => "hy",
            #[cfg(feature = "iso")]
            LANGUAGE::Basque => "eu",
            #[cfg(feature = "iso")]
            LANGUAGE::Bengali => "bn",
            #[cfg(feature = "iso")]
            LANGUAGE::Breton => "br",
            #[cfg(feature = "iso")]
            LANGUAGE::Bulgarian => "bg",
            #[cfg(feature = "iso")]
            LANGUAGE::Catalan => "ca",
            #[cfg(feature = "iso")]
            LANGUAGE::Czech => "cs",
            #[cfg(feature = "iso")]
            LANGUAGE::Chinese => "zh",
            #[cfg(feature = "iso")]
            LANGUAGE::Esperanto => "eo",
            #[cfg(feature = "iso")]
            LANGUAGE::Estonian => "et",
            #[cfg(feature = "iso")]
            LANGUAGE::Persian => "fa",
            #[cfg(feature = "iso")]
            LANGUAGE::Irish => "ga",
            #[cfg(feature = "iso")]
            LANGUAGE::Galician => "gl",
            #[cfg(feature = "iso")]
            LANGUAGE::Gujarati => "gu",
            #[cfg(feature = "iso")]
            LANGUAGE::Hausa => "ha",
            #[cfg(feature = "iso")]
            LANGUAGE::Hebrew => "he",
            #[cfg(feature = "iso")]
            LANGUAGE::Hindi => "hi",
            #[cfg(feature = "iso")]
            LANGUAGE::Croatian => "hr",
            #[cfg(feature = "iso")]
            LANGUAGE::Japanese => "ja",
            #[cfg(feature = "iso")]
            LANGUAGE::Korean => "ko",
            #[cfg(feature = "iso")]
            LANGUAGE::Kurdish => "ku",
            #[cfg(feature = "iso")]
            LANGUAGE::Latin => "la",
            #[cfg(feature = "iso")]
            LANGUAGE::Latvian => "lv",
            #[cfg(feature = "iso")]
            LANGUAGE::Lithuanian => "lt",
            #[cfg(feature = "iso")]
            LANGUAGE::Marathi => "mr",
            #[cfg(feature = "iso")]
            LANGUAGE::Malay => "ms",
            #[cfg(feature = "iso")]
            LANGUAGE::Polish => "pl",
            #[cfg(feature = "iso")]
            LANGUAGE::Slovak => "sk",
            #[cfg(feature = "iso")]
            LANGUAGE::Somali => "so",
            #[cfg(feature = "iso")]
            LANGUAGE::Sotho => "st",
            #[cfg(feature = "iso")]
            LANGUAGE::Swahili => "sw",
            #[cfg(feature = "iso")]
            LANGUAGE::Tagalog => "tl",
            #[cfg(feature = "iso")]
            LANGUAGE::Thai => "th",
            #[cfg(feature = "iso")]
            LANGUAGE::Ukrainian => "uk",
            #[cfg(feature = "iso")]
            LANGUAGE::Urdu => "ur",
            #[cfg(feature = "iso")]
            LANGUAGE::Vietnamese => "vi",
            #[cfg(feature = "iso")]
            LANGUAGE::Yoruba => "yo",
            #[cfg(feature = "iso")]
            LANGUAGE::Zulu => "zu",
            #[cfg(feature = "unimplemented")]
            LANGUAGE::Afar => "af",
        }
        .to_string()
    }
}

impl Display for LANGUAGE {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.clone())
    }
}
