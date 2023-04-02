//! Module containing the huge language enum and formatting for it

/// Enum containing available language names
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum LANGUAGE {
    #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
    /// Arabic (ISO 639-1 Code: ar)
    Arabic,

    #[cfg(feature = "nltk")]
    /// Azerbaijani (ISO 639-1 Code: az)
    Azerbaijani,

    #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
    /// Danish (ISO 639-1 Code: da)
    Danish,

    #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
    /// Dutch (ISO 639-1 Code: nl)
    Dutch,

    #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
    /// English (ISO 639-1 Code: en)
    English,

    #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
    /// Finnish (ISO 639-1 Code: fi)
    Finnish,

    #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
    /// French (ISO 639-1 Code: fr)
    French,

    #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
    /// German (ISO 639-1 Code: de)
    German,

    #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
    /// Greek (ISO 639-1 Code: el)
    Greek,

    #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
    /// Hungarian (ISO 639-1 Code: hu)
    Hungarian,

    #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
    /// Indonesian (ISO 639-1 Code: id)
    Indonesian,

    #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
    /// Italian (ISO 639-1 Code: it)
    Italian,

    #[cfg(feature = "nltk")]
    /// Kazakh (ISO 639-1 Code: kk)
    Kazakh,

    #[cfg(feature = "nltk")]
    /// Nepali (ISO 639-1 Code: ne)
    Nepali,

    #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
    /// Norwegian (ISO 639-1 Code: no)
    Norwegian,

    #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
    /// Portuguese (ISO 639-1 Code: pt)
    Portuguese,

    #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
    /// Romanian (ISO 639-1 Code: ro)
    Romanian,

    #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
    /// Russian (ISO 639-1 Code: ru)
    Russian,

    #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
    /// Slovenian (ISO 639-1 Code: sl)
    Slovenian,

    #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
    /// Spanish (ISO 639-1 Code: sp)
    Spanish,

    #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
    /// Swedish (ISO 639-1 Code: sv)
    Swedish,

    #[cfg(feature = "nltk")]
    /// Tajik (ISO 639-1 Code: tg)
    Tajik,

    #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
    /// Turkish (ISO 639-1 Code: tk)
    Turkish,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Afrikaans (ISO 639-1 Code: af)
    Afrikaans,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Armenian (ISO 639-1 Code: hy)
    Armenian,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Basque (ISO 639-1 Code: eu)
    Basque,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Bengali (ISO 639-1 Code: bn)
    Bengali,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Breton (ISO 639-1 Code: br)
    Breton,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Bulgarian (ISO 639-1 Code: bg)
    Bulgarian,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Catalan (ISO 639-1 Code: ca)
    Catalan,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Czech (ISO 639-1 Code: cs)
    Czech,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Chinese (ISO 639-1 Code: zh)
    Chinese,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Esperanto (ISO 639-1 Code: eo)
    Esperanto,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Estonian (ISO 639-1 Code: eo)
    Estonian,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Persian (ISO 639-1 Code: fa)
    Persian,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Irish (ISO 639-1 Code: ga)
    Irish,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Galician (ISO 639-1 Code: gl)
    Galician,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Gujarati (ISO 639-1 Code: gu)
    Gujarati,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Hausa (ISO 639-1 Code: ha)
    Hausa,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Hebrew (ISO 639-1 Code: he)
    Hebrew,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Hindi (ISO 639-1 Code: hi)
    Hindi,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Croatian (ISO 639-1 Code: hr)
    Croatian,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Japanese (ISO 639-1 Code: ha)
    Japanese,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Korean (ISO 639-1 Code: ko)
    Korean,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Kurdish (ISO 639-1 Code: ku)
    Kurdish,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Latin (ISO 639-1 Code: la)
    Latin,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Latvian (ISO 639-1 Code: lv)
    Latvian,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Lithuanian (ISO 639-1 Code: lt)
    Lithuanian,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Marathi (ISO 639-1 Code: mr)
    Marathi,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Malay (ISO 639-1 Code: ms)
    Malay,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Polish (ISO 639-1 Code: pl)
    Polish,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Slovak (ISO 639-1 Code: sk)
    Slovak,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Somali (ISO 639-1 Code: so)
    Somali,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Sotho (ISO 639-1 Code: st)
    Sotho,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Swahili (ISO 639-1 Code: sw)
    Swahili,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Taglog (ISO 639-1 Code: tl)
    Tagalog,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Thai (ISO 639-1 Code: th)
    Thai,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Ukrainian (ISO 639-1 Code: uk)
    Ukrainian,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Urdu (ISO 639-1 Code: ur)
    Urdu,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Vietnamese (ISO 639-1 Code: vi)
    Vietnamese,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Yoruba (ISO 639-1 Code: yo)
    Yoruba,

    #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
    /// Zulu (ISO 639-1 Code: zu)
    Zulu,

    #[cfg(feature = "unimplemented")]
    /// Afar (ISO 639-1 Code: aa)
    Afar,

    #[cfg(feature = "constructed")]
    /// Quenya (ISO 639-3 Code: qya)
    Quenya,

    #[cfg(feature = "constructed")]
    /// Sindarin (ISO 639-3 Code: sjn)
    Sindarin,

    #[cfg(feature = "constructed")]
    /// Klingon (ISO 639-3 Code: tlh)
    Klingon,

    #[cfg(feature = "constructed")]
    /// Dothraki (ISO 639-3 Code: N/A, so _dot_ is used here)
    Dothraki,

    #[cfg(feature = "constructed")]
    /// Dovahzul (ISO 639-3 Code: N/A, so _dov_ is used here)
    Dovahzul,

    #[cfg(feature = "constructed")]
    /// Navi (ISO 639-3 Code: N/A, so _nav_ is used here)
    Navi,

    #[cfg(feature = "constructed")]
    /// High Valyrian (ISO 639-3 Code: N/A, so _val_ is used here)
    HighValyrian,
}

impl From<LANGUAGE> for String {
    fn from(value: LANGUAGE) -> Self {
        match value {
            #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
            LANGUAGE::Arabic => "ar",
            #[cfg(feature = "nltk")]
            LANGUAGE::Azerbaijani => "az",
            #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
            LANGUAGE::Danish => "da",
            #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
            LANGUAGE::Dutch => "nl",
            #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
            LANGUAGE::English => "en",
            #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
            LANGUAGE::Finnish => "fi",
            #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
            LANGUAGE::French => "fr",
            #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
            LANGUAGE::German => "de",
            #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
            LANGUAGE::Greek => "el",
            #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
            LANGUAGE::Hungarian => "hu",
            #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
            LANGUAGE::Indonesian => "id",
            #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
            LANGUAGE::Italian => "it",
            #[cfg(feature = "nltk")]
            LANGUAGE::Kazakh => "kk",
            #[cfg(feature = "nltk")]
            LANGUAGE::Nepali => "ne",
            #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
            LANGUAGE::Norwegian => "no",
            #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
            LANGUAGE::Portuguese => "pt",
            #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
            LANGUAGE::Romanian => "ro",
            #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
            LANGUAGE::Russian => "ru",
            #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
            LANGUAGE::Slovenian => "sl",
            #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
            LANGUAGE::Spanish => "es",
            #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
            LANGUAGE::Swedish => "sv",
            #[cfg(feature = "nltk")]
            LANGUAGE::Tajik => "tg",
            #[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
            LANGUAGE::Turkish => "tr",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Afrikaans => "af",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Armenian => "hy",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Basque => "eu",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Bengali => "bn",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Breton => "br",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Bulgarian => "bg",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Catalan => "ca",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Czech => "cs",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Chinese => "zh",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Esperanto => "eo",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Estonian => "et",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Persian => "fa",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Irish => "ga",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Galician => "gl",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Gujarati => "gu",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Hausa => "ha",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Hebrew => "he",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Hindi => "hi",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Croatian => "hr",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Japanese => "ja",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Korean => "ko",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Kurdish => "ku",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Latin => "la",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Latvian => "lv",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Lithuanian => "lt",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Marathi => "mr",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Malay => "ms",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Polish => "pl",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Slovak => "sk",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Somali => "so",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Sotho => "st",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Swahili => "sw",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Tagalog => "tl",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Thai => "th",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Ukrainian => "uk",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Urdu => "ur",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Vietnamese => "vi",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Yoruba => "yo",
            #[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
            LANGUAGE::Zulu => "zu",
            #[cfg(feature = "unimplemented")]
            LANGUAGE::Afar => "aa",
            #[cfg(feature = "constructed")]
            LANGUAGE::Quenya => "qya",
            #[cfg(feature = "constructed")]
            LANGUAGE::Sindarin => "sjn",
            #[cfg(feature = "constructed")]
            LANGUAGE::Klingon => "tlh",
            #[cfg(feature = "constructed")]
            LANGUAGE::Dothraki => "dot",
            #[cfg(feature = "constructed")]
            LANGUAGE::Dovahzul => "dov",
            #[cfg(feature = "constructed")]
            LANGUAGE::Navi => "nav",
            #[cfg(feature = "constructed")]
            LANGUAGE::HighValyrian => "val",
        }
        .to_string()
    }
}

impl std::fmt::Display for LANGUAGE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.clone())
    }
}
