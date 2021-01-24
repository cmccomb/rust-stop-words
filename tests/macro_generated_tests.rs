// use stop_words;

/// Let's define a macro to help us out
macro_rules! test {
    (
        $language_full:literal,
        $language_2:literal,
        $language_3:literal,
        $nltk_panic:ident
    ) => {
        #[cfg(feature = "enum")]
        use std::str::FromStr;

        #[test]
        #[cfg(not(feature = "enum"))]
        fn compare_full_to_2letter() {
            let s = $language_full;
            let x = stop_words::get(Box::leak(s.to_lowercase().into_boxed_str()));
            let y = stop_words::get($language_2);
            for idx in 0..x.len() {
                assert_eq!(x[idx], y[idx])
            }
        }

        #[test]
        #[cfg(not(feature = "enum"))]
        fn compare_full_to_3letter() {
            let s = $language_full;
            let x = stop_words::get(Box::leak(s.to_lowercase().into_boxed_str()));
            let y = stop_words::get($language_3);
            for idx in 0..x.len() {
                assert_eq!(x[idx], y[idx])
            }
        }

        #[test]
        #[cfg(feature = "enum")]
        fn check_enum() {
            let x = stop_words::get(stop_words::LANGUAGE::from_str($language_full).unwrap());

            for idx in 0..x.len() {
                println!("{}", x[idx])
            }
        }

        #[test]
        #[cfg(not(feature = "enum"))]
        #[$nltk_panic]
        fn check_nltk() {
            let s = $language_full;
            let x = stop_words::get_nltk(Box::leak(s.to_lowercase().into_boxed_str()));

            for idx in 0..x.len() {
                println!("{}", x[idx])
            }
        }
    };
}

#[cfg(test)]
mod arabic {
    test!("Arabic", "ar", "ara", test);
}

#[cfg(test)]
mod azerbaijani {
    test!("Azerbaijani", "az", "aze", test);
}

#[cfg(test)]
mod bulgarian {
    test!("Bulgarian", "bg", "bul", should_panic);
}

#[cfg(test)]
mod catalan {
    test!("Catalan", "ca", "cat", should_panic);
}

#[cfg(test)]
mod czech {
    test!("Czech", "cs", "ces", should_panic);
}

#[cfg(test)]
mod danish {
    test!("Danish", "da", "dan", test);
}

#[cfg(test)]
mod dutch {
    test!("Dutch", "nl", "nld", test);
}

#[cfg(test)]
mod english {
    test!("English", "en", "eng", test);
}

#[cfg(test)]
mod finnish {
    test!("Finnish", "fi", "fin", test);
}

#[cfg(test)]
mod french {
    test!("French", "fr", "fra", test);
}

#[cfg(test)]
mod german {
    test!("German", "de", "deu", test);
}

#[cfg(test)]
mod greek {
    test!("Greek", "el", "ell", test);
}

#[cfg(test)]
mod gujarati {
    test!("Gujarati", "gu", "guj", should_panic);
}

#[cfg(test)]
mod hebrew {
    test!("Hebrew", "he", "heb", should_panic);
}

#[cfg(test)]
mod hindi {
    test!("Hindi", "hi", "hin", should_panic);
}

#[cfg(test)]
mod hungarian {
    test!("Hungarian", "hu", "hun", test);
}

#[cfg(test)]
mod indonesian {
    test!("Indonesian", "in", "ind", test);
}

#[cfg(test)]
mod italian {
    test!("Italian", "it", "ita", test);
}

#[cfg(test)]
mod kazakh {
    test!("Kazakh", "kk", "kaz", test);
}

#[cfg(test)]
mod nepali {
    test!("Nepali", "ne", "nep", test);
}

#[cfg(test)]
mod norwegian {
    test!("Norwegian", "nn", "nno", test);
}

#[cfg(test)]
mod polish {
    test!("Polish", "pl", "pol", should_panic);
}

#[cfg(test)]
mod portuguese {
    test!("Portuguese", "pt", "por", test);
}

#[cfg(test)]
mod romanian {
    test!("Romanian", "ro", "ron", test);
}

#[cfg(test)]
mod russian {
    test!("Russian", "ru", "rus", test);
}

#[cfg(test)]
mod slovak {
    test!("Slovak", "sk", "slk", should_panic);
}

#[cfg(test)]
mod slovenian {
    test!("Slovenian", "sl", "slv", test);
}

#[cfg(test)]
mod spanish {
    test!("Spanish", "es", "spa", test);
}

#[cfg(test)]
mod swedish {
    test!("Swedish", "sv", "swe", test);
}

#[cfg(test)]
mod tajik {
    test!("Tajik", "tg", "tgk", test);
}

#[cfg(test)]
mod turkish {
    test!("Turkish", "tr", "tur", test);
}

#[cfg(test)]
mod ukrainian {
    test!("Ukrainian", "uk", "ukr", should_panic);
}

#[cfg(test)]
mod vietnamese {
    test!("Vietnamese", "vi", "vie", should_panic);
}
