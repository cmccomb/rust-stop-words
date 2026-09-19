//! Language enums and conversions, including the deprecated compatibility API.

/// Defines a language enum and its conversions with shared variants and feature gates.
macro_rules! define_language {
    ($(#[$attr:meta])* $name:ident) => {
        /// Enum containing available language names
        #[non_exhaustive]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        $(#[$attr])*
        pub enum $name {
            #[cfg(feature = "nltk")]
            /// Albanian (ISO 639-1 Code: sq)
            Albanian,

            #[cfg(any(feature = "nltk", feature = "iso"))]
            /// Arabic (ISO 639-1 Code: ar)
            Arabic,

            #[cfg(feature = "nltk")]
            /// Azerbaijani (ISO 639-1 Code: az)
            Azerbaijani,

            #[cfg(any(feature = "nltk", feature = "iso"))]
            /// Danish (ISO 639-1 Code: da)
            Danish,

            #[cfg(any(feature = "nltk", feature = "iso"))]
            /// Dutch (ISO 639-1 Code: nl)
            Dutch,

            #[cfg(any(feature = "nltk", feature = "iso"))]
            /// English (ISO 639-1 Code: en)
            English,

            #[cfg(any(feature = "nltk", feature = "iso"))]
            /// Finnish (ISO 639-1 Code: fi)
            Finnish,

            #[cfg(any(feature = "nltk", feature = "iso"))]
            /// French (ISO 639-1 Code: fr)
            French,

            #[cfg(any(feature = "nltk", feature = "iso"))]
            /// German (ISO 639-1 Code: de)
            German,

            #[cfg(any(feature = "nltk", feature = "iso"))]
            /// Greek (ISO 639-1 Code: el)
            Greek,

            #[cfg(any(feature = "nltk", feature = "iso"))]
            /// Hungarian (ISO 639-1 Code: hu)
            Hungarian,

            #[cfg(any(feature = "nltk", feature = "iso"))]
            /// Indonesian (ISO 639-1 Code: id)
            Indonesian,

            #[cfg(any(feature = "nltk", feature = "iso"))]
            /// Italian (ISO 639-1 Code: it)
            Italian,

            #[cfg(feature = "nltk")]
            /// Kazakh (ISO 639-1 Code: kk)
            Kazakh,

            #[cfg(feature = "nltk")]
            /// Nepali (ISO 639-1 Code: ne)
            Nepali,

            #[cfg(any(feature = "nltk", feature = "iso"))]
            /// Norwegian (ISO 639-1 Code: no)
            Norwegian,

            #[cfg(any(feature = "nltk", feature = "iso"))]
            /// Portuguese (ISO 639-1 Code: pt)
            Portuguese,

            #[cfg(any(feature = "nltk", feature = "iso"))]
            /// Romanian (ISO 639-1 Code: ro)
            Romanian,

            #[cfg(any(feature = "nltk", feature = "iso"))]
            /// Russian (ISO 639-1 Code: ru)
            Russian,

            #[cfg(any(feature = "nltk", feature = "iso"))]
            /// Slovenian (ISO 639-1 Code: sl)
            Slovenian,

            #[cfg(any(feature = "nltk", feature = "iso"))]
            /// Spanish (ISO 639-1 Code: es)
            Spanish,

            #[cfg(any(feature = "nltk", feature = "iso"))]
            /// Swedish (ISO 639-1 Code: sv)
            Swedish,

            #[cfg(feature = "nltk")]
            /// Tajik (ISO 639-1 Code: tg)
            Tajik,

            #[cfg(feature = "nltk")]
            /// Tamil (ISO 639-1 Code: ta)
            Tamil,

            #[cfg(any(feature = "nltk", feature = "iso"))]
            /// Turkish (ISO 639-1 Code: tr)
            Turkish,

            #[cfg(feature = "nltk")]
            /// Uzbek (ISO 639-1 Code: uz)
            Uzbek,

            #[cfg(feature = "iso")]
            /// Afrikaans (ISO 639-1 Code: af)
            Afrikaans,

            #[cfg(feature = "iso")]
            /// Armenian (ISO 639-1 Code: hy)
            Armenian,

            #[cfg(any(feature = "nltk", feature = "iso"))]
            /// Basque (ISO 639-1 Code: eu)
            Basque,

            #[cfg(feature = "nltk")]
            /// Belarusian (ISO 639-1 Code: be)
            Belarusian,

            #[cfg(any(feature = "nltk", feature = "iso"))]
            /// Bengali (ISO 639-1 Code: bn)
            Bengali,

            #[cfg(feature = "iso")]
            /// Breton (ISO 639-1 Code: br)
            Breton,

            #[cfg(feature = "iso")]
            /// Bulgarian (ISO 639-1 Code: bg)
            Bulgarian,

            #[cfg(any(feature = "nltk", feature = "iso"))]
            /// Catalan (ISO 639-1 Code: ca)
            Catalan,

            #[cfg(feature = "iso")]
            /// Czech (ISO 639-1 Code: cs)
            Czech,

            #[cfg(any(feature = "nltk", feature = "iso"))]
            /// Chinese (ISO 639-1 Code: zh)
            Chinese,

            #[cfg(feature = "iso")]
            /// Esperanto (ISO 639-1 Code: eo)
            Esperanto,

            #[cfg(feature = "iso")]
            /// Estonian (ISO 639-1 Code: et)
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

            #[cfg(any(feature = "nltk", feature = "iso"))]
            /// Hebrew (ISO 639-1 Code: he)
            Hebrew,

            #[cfg(feature = "nltk")]
            /// Hinglish (NLTK-specific identifier: hinglish)
            Hinglish,

            #[cfg(feature = "iso")]
            /// Hindi (ISO 639-1 Code: hi)
            Hindi,

            #[cfg(feature = "iso")]
            /// Croatian (ISO 639-1 Code: hr)
            Croatian,

            #[cfg(feature = "iso")]
            /// Japanese (ISO 639-1 Code: ja)
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
            /// Tagalog (ISO 639-1 Code: tl)
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

        impl $name {
            /// Return the lookup language code for this variant.
            #[must_use]
            #[allow(clippy::too_many_lines)]
            pub const fn as_str(&self) -> &'static str {
                match self {
                    #[cfg(feature = "nltk")]
                    $name::Albanian => "sq",
                    #[cfg(any(feature = "nltk", feature = "iso"))]
                    $name::Arabic => "ar",
                    #[cfg(feature = "nltk")]
                    $name::Azerbaijani => "az",
                    #[cfg(any(feature = "nltk", feature = "iso"))]
                    $name::Danish => "da",
                    #[cfg(any(feature = "nltk", feature = "iso"))]
                    $name::Dutch => "nl",
                    #[cfg(any(feature = "nltk", feature = "iso"))]
                    $name::English => "en",
                    #[cfg(any(feature = "nltk", feature = "iso"))]
                    $name::Finnish => "fi",
                    #[cfg(any(feature = "nltk", feature = "iso"))]
                    $name::French => "fr",
                    #[cfg(any(feature = "nltk", feature = "iso"))]
                    $name::German => "de",
                    #[cfg(any(feature = "nltk", feature = "iso"))]
                    $name::Greek => "el",
                    #[cfg(any(feature = "nltk", feature = "iso"))]
                    $name::Hungarian => "hu",
                    #[cfg(any(feature = "nltk", feature = "iso"))]
                    $name::Indonesian => "id",
                    #[cfg(any(feature = "nltk", feature = "iso"))]
                    $name::Italian => "it",
                    #[cfg(feature = "nltk")]
                    $name::Kazakh => "kk",
                    #[cfg(feature = "nltk")]
                    $name::Nepali => "ne",
                    #[cfg(any(feature = "nltk", feature = "iso"))]
                    $name::Norwegian => "no",
                    #[cfg(any(feature = "nltk", feature = "iso"))]
                    $name::Portuguese => "pt",
                    #[cfg(any(feature = "nltk", feature = "iso"))]
                    $name::Romanian => "ro",
                    #[cfg(any(feature = "nltk", feature = "iso"))]
                    $name::Russian => "ru",
                    #[cfg(any(feature = "nltk", feature = "iso"))]
                    $name::Slovenian => "sl",
                    #[cfg(any(feature = "nltk", feature = "iso"))]
                    $name::Spanish => "es",
                    #[cfg(any(feature = "nltk", feature = "iso"))]
                    $name::Swedish => "sv",
                    #[cfg(feature = "nltk")]
                    $name::Tajik => "tg",
                    #[cfg(feature = "nltk")]
                    $name::Tamil => "ta",
                    #[cfg(any(feature = "nltk", feature = "iso"))]
                    $name::Turkish => "tr",
                    #[cfg(feature = "nltk")]
                    $name::Uzbek => "uz",
                    #[cfg(feature = "iso")]
                    $name::Afrikaans => "af",
                    #[cfg(feature = "iso")]
                    $name::Armenian => "hy",
                    #[cfg(any(feature = "nltk", feature = "iso"))]
                    $name::Basque => "eu",
                    #[cfg(feature = "nltk")]
                    $name::Belarusian => "be",
                    #[cfg(any(feature = "nltk", feature = "iso"))]
                    $name::Bengali => "bn",
                    #[cfg(feature = "iso")]
                    $name::Breton => "br",
                    #[cfg(feature = "iso")]
                    $name::Bulgarian => "bg",
                    #[cfg(any(feature = "nltk", feature = "iso"))]
                    $name::Catalan => "ca",
                    #[cfg(feature = "iso")]
                    $name::Czech => "cs",
                    #[cfg(any(feature = "nltk", feature = "iso"))]
                    $name::Chinese => "zh",
                    #[cfg(feature = "iso")]
                    $name::Esperanto => "eo",
                    #[cfg(feature = "iso")]
                    $name::Estonian => "et",
                    #[cfg(feature = "iso")]
                    $name::Persian => "fa",
                    #[cfg(feature = "iso")]
                    $name::Irish => "ga",
                    #[cfg(feature = "iso")]
                    $name::Galician => "gl",
                    #[cfg(feature = "iso")]
                    $name::Gujarati => "gu",
                    #[cfg(feature = "iso")]
                    $name::Hausa => "ha",
                    #[cfg(any(feature = "nltk", feature = "iso"))]
                    $name::Hebrew => "he",
                    #[cfg(feature = "nltk")]
                    $name::Hinglish => "hinglish",
                    #[cfg(feature = "iso")]
                    $name::Hindi => "hi",
                    #[cfg(feature = "iso")]
                    $name::Croatian => "hr",
                    #[cfg(feature = "iso")]
                    $name::Japanese => "ja",
                    #[cfg(feature = "iso")]
                    $name::Korean => "ko",
                    #[cfg(feature = "iso")]
                    $name::Kurdish => "ku",
                    #[cfg(feature = "iso")]
                    $name::Latin => "la",
                    #[cfg(feature = "iso")]
                    $name::Latvian => "lv",
                    #[cfg(feature = "iso")]
                    $name::Lithuanian => "lt",
                    #[cfg(feature = "iso")]
                    $name::Marathi => "mr",
                    #[cfg(feature = "iso")]
                    $name::Malay => "ms",
                    #[cfg(feature = "iso")]
                    $name::Polish => "pl",
                    #[cfg(feature = "iso")]
                    $name::Slovak => "sk",
                    #[cfg(feature = "iso")]
                    $name::Somali => "so",
                    #[cfg(feature = "iso")]
                    $name::Sotho => "st",
                    #[cfg(feature = "iso")]
                    $name::Swahili => "sw",
                    #[cfg(feature = "iso")]
                    $name::Tagalog => "tl",
                    #[cfg(feature = "iso")]
                    $name::Thai => "th",
                    #[cfg(feature = "iso")]
                    $name::Ukrainian => "uk",
                    #[cfg(feature = "iso")]
                    $name::Urdu => "ur",
                    #[cfg(feature = "iso")]
                    $name::Vietnamese => "vi",
                    #[cfg(feature = "iso")]
                    $name::Yoruba => "yo",
                    #[cfg(feature = "iso")]
                    $name::Zulu => "zu",
                    #[cfg(feature = "unimplemented")]
                    $name::Afar => "aa",
                    #[cfg(feature = "constructed")]
                    $name::Quenya => "qya",
                    #[cfg(feature = "constructed")]
                    $name::Sindarin => "sjn",
                    #[cfg(feature = "constructed")]
                    $name::Klingon => "tlh",
                    #[cfg(feature = "constructed")]
                    $name::Dothraki => "dot",
                    #[cfg(feature = "constructed")]
                    $name::Dovahzul => "dov",
                    #[cfg(feature = "constructed")]
                    $name::Navi => "nav",
                    #[cfg(feature = "constructed")]
                    $name::HighValyrian => "val",
                    #[allow(unreachable_patterns)]
                    _ => panic!("no languages are enabled"),
                }
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.as_str().to_owned()
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_ref())
            }
        }

    };
}

define_language!(Language);

/// Preserves the original enum and variant imports while deprecating their use.
#[allow(deprecated)]
mod legacy {
    define_language!(
        /// Use [`crate::Language`] for new code. Existing variant imports remain supported.
        ///
        /// Treating deprecations as errors rejects the original name:
        ///
        /// ```compile_fail
        /// #![deny(deprecated)]
        /// use stop_words::LANGUAGE;
        /// ```
        #[deprecated(since = "0.10.1", note = "use `Language` instead")]
        LANGUAGE
    );
}

#[allow(deprecated)]
pub use legacy::LANGUAGE;
