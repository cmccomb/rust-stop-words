//! Module containing the huge language enum and formatting for it

use std::fmt::Display;
use std::fmt::Formatter;

/// Enum containing available language names
#[non_exhaustive]
pub enum LANGUAGE {
    /// Arabic (ISO 639-1 Code: ar)
    Arabic,

    #[cfg(feature = "nltk")]
    /// Azerbaijani (ISO 639-1 Code: az)
    Azerbaijani,

    /// Danish (ISO 639-1 Code: da)
    Danish,

    /// Dutch (ISO 639-1 Code: nl)
    Dutch,

    /// English (ISO 639-1 Code: en)
    English,

    /// Finnish (ISO 639-1 Code: fi)
    Finnish,

    /// French (ISO 639-1 Code: fr)
    French,

    /// German (ISO 639-1 Code: de)
    German,

    /// Greek (ISO 639-1 Code: el)
    Greek,

    /// Hungarian (ISO 639-1 Code: hu)
    Hungarian,

    /// Indonesian (ISO 639-1 Code: id)
    Indonesian,

    /// Italian (ISO 639-1 Code: it)
    Italian,

    #[cfg(feature = "nltk")]
    /// Kazakh (ISO 639-1 Code: kk)
    Kazakh,

    #[cfg(feature = "nltk")]
    /// Nepali (ISO 639-1 Code: ne)
    Nepali,

    /// Norwegian (ISO 639-1 Code: no)
    Norwegian,

    /// Portuguese (ISO 639-1 Code: pt)
    Portuguese,

    /// Romanian (ISO 639-1 Code: ro)
    Romanian,

    /// Russian (ISO 639-1 Code: ru)
    Russian,

    /// Slovenian (ISO 639-1 Code: sl)
    Slovenian,

    /// Spanish (ISO 639-1 Code: sp)
    Spanish,

    /// Swedish (ISO 639-1 Code: sv)
    Swedish,

    #[cfg(feature = "nltk")]
    /// Tajik (ISO 639-1 Code: tg)
    Tajik,

    /// Turkish (ISO 639-1 Code: tk)
    Turkish,

    #[cfg(not(feature = "nltk"))]
    /// Afrikaans (ISO 639-1 Code: af)
    Afrikaans,

    #[cfg(not(feature = "nltk"))]
    /// Armenian (ISO 639-1 Code: hy)
    Armenian,

    #[cfg(not(feature = "nltk"))]
    /// Basque (ISO 639-1 Code: eu)
    Basque,

    #[cfg(not(feature = "nltk"))]
    /// Bengali (ISO 639-1 Code: bn)
    Bengali,

    #[cfg(not(feature = "nltk"))]
    /// Breton (ISO 639-1 Code: br)
    Breton,

    #[cfg(not(feature = "nltk"))]
    /// Bulgarian (ISO 639-1 Code: bg)
    Bulgarian,

    #[cfg(not(feature = "nltk"))]
    /// Catalan (ISO 639-1 Code: ca)
    Catalan,

    #[cfg(not(feature = "nltk"))]
    /// Czech (ISO 639-1 Code: cs)
    Czech,

    #[cfg(not(feature = "nltk"))]
    /// Chinese (ISO 639-1 Code: zh)
    Chinese,

    #[cfg(not(feature = "nltk"))]
    /// Esperanto (ISO 639-1 Code: eo)
    Esperanto,

    #[cfg(not(feature = "nltk"))]
    /// Estonian (ISO 639-1 Code: eo)
    Estonian,

    #[cfg(not(feature = "nltk"))]
    /// Persian (ISO 639-1 Code: fa)
    Persian,

    #[cfg(not(feature = "nltk"))]
    /// Irish (ISO 639-1 Code: ga)
    Irish,

    #[cfg(not(feature = "nltk"))]
    /// Galician (ISO 639-1 Code: gl)
    Galician,

    #[cfg(not(feature = "nltk"))]
    /// Gujarati (ISO 639-1 Code: gu)
    Gujarati,

    #[cfg(not(feature = "nltk"))]
    /// Hausa (ISO 639-1 Code: ha)
    Hausa,

    #[cfg(not(feature = "nltk"))]
    /// Hebrew (ISO 639-1 Code: he)
    Hebrew,

    #[cfg(not(feature = "nltk"))]
    /// Hindi (ISO 639-1 Code: hi)
    Hindi,

    #[cfg(not(feature = "nltk"))]
    /// Croatian (ISO 639-1 Code: hr)
    Croatian,

    #[cfg(not(feature = "nltk"))]
    /// Japanese (ISO 639-1 Code: ha)
    Japanese,

    #[cfg(not(feature = "nltk"))]
    /// Korean (ISO 639-1 Code: ko)
    Korean,

    #[cfg(not(feature = "nltk"))]
    /// Kurdish (ISO 639-1 Code: ku)
    Kurdish,

    #[cfg(not(feature = "nltk"))]
    /// Latin (ISO 639-1 Code: la)
    Latin,

    #[cfg(not(feature = "nltk"))]
    /// Latvian (ISO 639-1 Code: lv)
    Latvian,

    #[cfg(not(feature = "nltk"))]
    /// Lithuanian (ISO 639-1 Code: lt)
    Lithuanian,

    #[cfg(not(feature = "nltk"))]
    /// Marathi (ISO 639-1 Code: mr)
    Marathi,

    #[cfg(not(feature = "nltk"))]
    /// Malay (ISO 639-1 Code: ms)
    Malay,

    #[cfg(not(feature = "nltk"))]
    /// Polish (ISO 639-1 Code: pl)
    Polish,

    #[cfg(not(feature = "nltk"))]
    /// Slovak (ISO 639-1 Code: sk)
    Slovak,

    #[cfg(not(feature = "nltk"))]
    /// Somali (ISO 639-1 Code: so)
    Somali,

    #[cfg(not(feature = "nltk"))]
    /// Sotho (ISO 639-1 Code: st)
    Sotho,

    #[cfg(not(feature = "nltk"))]
    /// Swahili (ISO 639-1 Code: sw)
    Swahili,

    #[cfg(not(feature = "nltk"))]
    /// Taglog (ISO 639-1 Code: tl)
    Tagalog,

    #[cfg(not(feature = "nltk"))]
    /// Thai (ISO 639-1 Code: th)
    Thai,

    #[cfg(not(feature = "nltk"))]
    /// Ukrainian (ISO 639-1 Code: uk)
    Ukrainian,

    #[cfg(not(feature = "nltk"))]
    /// Urdu (ISO 639-1 Code: ur)
    Urdu,

    #[cfg(not(feature = "nltk"))]
    /// Vietnamese (ISO 639-1 Code: vi)
    Vietnamese,

    #[cfg(not(feature = "nltk"))]
    /// Yoruba (ISO 639-1 Code: yo)
    Yoruba,

    #[cfg(not(feature = "nltk"))]
    /// Zulu (ISO 639-1 Code: zu)
    Zulu,

    #[cfg(feature = "unimplemented")]
    /// Afar (ISO 639-1 Code: af)
    Afar,
}

// #[cfg(feature = "nltk")]
impl Display for LANGUAGE {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LANGUAGE::Arabic => write!(f, "ar"),
            #[cfg(feature = "nltk")]
            LANGUAGE::Azerbaijani => write!(f, "az"),
            LANGUAGE::Danish => write!(f, "da"),
            LANGUAGE::Dutch => write!(f, "nl"),
            LANGUAGE::English => write!(f, "en"),
            LANGUAGE::Finnish => write!(f, "fi"),
            LANGUAGE::French => write!(f, "fr"),
            LANGUAGE::German => write!(f, "de"),
            LANGUAGE::Greek => write!(f, "el"),
            LANGUAGE::Hungarian => write!(f, "hu"),
            LANGUAGE::Indonesian => write!(f, "id"),
            LANGUAGE::Italian => write!(f, "it"),
            #[cfg(feature = "nltk")]
            LANGUAGE::Kazakh => write!(f, "kk"),
            #[cfg(feature = "nltk")]
            LANGUAGE::Nepali => write!(f, "ne"),
            LANGUAGE::Norwegian => write!(f, "no"),
            LANGUAGE::Portuguese => write!(f, "pt"),
            LANGUAGE::Romanian => write!(f, "ro"),
            LANGUAGE::Russian => write!(f, "ru"),
            LANGUAGE::Slovenian => write!(f, "sl"),
            LANGUAGE::Spanish => write!(f, "es"),
            LANGUAGE::Swedish => write!(f, "sv"),
            #[cfg(feature = "nltk")]
            LANGUAGE::Tajik => write!(f, "tg"),
            LANGUAGE::Turkish => write!(f, "tr"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Afrikaans => write!(f, "af"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Armenian => write!(f, "hy"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Basque => write!(f, "eu"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Bengali => write!(f, "bn"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Breton => write!(f, "br"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Bulgarian => write!(f, "bg"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Catalan => write!(f, "ca"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Czech => write!(f, "cs"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Chinese => write!(f, "zh"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Esperanto => write!(f, "eo"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Estonian => write!(f, "et"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Persian => write!(f, "fa"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Irish => write!(f, "ga"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Galician => write!(f, "gl"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Gujarati => write!(f, "gu"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Hausa => write!(f, "ha"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Hebrew => write!(f, "he"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Hindi => write!(f, "hi"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Croatian => write!(f, "hr"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Japanese => write!(f, "ja"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Korean => write!(f, "ko"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Kurdish => write!(f, "ku"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Latin => write!(f, "la"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Latvian => write!(f, "lv"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Lithuanian => write!(f, "lt"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Marathi => write!(f, "mr"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Malay => write!(f, "ms"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Polish => write!(f, "pl"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Slovak => write!(f, "sk"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Somali => write!(f, "so"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Sotho => write!(f, "st"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Swahili => write!(f, "sw"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Tagalog => write!(f, "tl"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Thai => write!(f, "th"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Ukrainian => write!(f, "uk"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Urdu => write!(f, "ur"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Vietnamese => write!(f, "vi"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Yoruba => write!(f, "yo"),
            #[cfg(not(feature = "nltk"))]
            LANGUAGE::Zulu => write!(f, "zu"),
            #[cfg(feature = "unimplemented")]
            LANGUAGE::Afar => write!(f, "af"),
        }
    }
}
