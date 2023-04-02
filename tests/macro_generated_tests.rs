/// Let's define a macro to help us out
macro_rules! test {
    (
        $language_full:expr
    ) => {
        #[test]
        fn compare_enum_to_2letter() {
            // Pul out the name versions that we want
            let lingo = $language_full;
            let lingo_as_enum = lingo.clone();
            let lingo_as_string: String = lingo.clone().into();
            let lingo_as_str = &*(lingo_as_string.clone());

            // Pull word lists
            let word_list_from_enum = stop_words::get(lingo_as_enum);
            let word_list_from_string = stop_words::get(lingo_as_string);
            let word_list_from_str = stop_words::get(lingo_as_str);

            // Run a whole hell of a lot of assertions
            for idx in 0..word_list_from_enum.len() {
                assert_eq!(word_list_from_enum[idx], word_list_from_string[idx]);
                assert_eq!(word_list_from_str[idx], word_list_from_string[idx]);
                assert_eq!(word_list_from_enum[idx], word_list_from_str[idx]);
            }
        }

        #[test]
        fn make_sure_list_is_not_empty() {
            let x = stop_words::get($language_full);
            assert!(x.len() > 0)
        }
    };
}

#[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
#[cfg(test)]
mod arabic {
    test!(stop_words::LANGUAGE::Arabic);
}

#[cfg(feature = "nltk")]
#[cfg(test)]
mod azerbaijani {
    test!(stop_words::LANGUAGE::Azerbaijani);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod afrikaans {
    test!(stop_words::LANGUAGE::Afrikaans);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod armenian {
    test!(stop_words::LANGUAGE::Armenian);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod basque {
    test!(stop_words::LANGUAGE::Basque);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod bengali {
    test!(stop_words::LANGUAGE::Bengali);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod breton {
    test!(stop_words::LANGUAGE::Breton);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod bulgarian {
    test!(stop_words::LANGUAGE::Bulgarian);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod catalan {
    test!(stop_words::LANGUAGE::Catalan);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod czech {
    test!(stop_words::LANGUAGE::Czech);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod chinese {
    test!(stop_words::LANGUAGE::Chinese);
}

#[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
#[cfg(test)]
mod danish {
    test!(stop_words::LANGUAGE::Danish);
}

#[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
#[cfg(test)]
mod dutch {
    test!(stop_words::LANGUAGE::Dutch);
}

#[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
#[cfg(test)]
mod english {
    test!(stop_words::LANGUAGE::English);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod esperanto {
    test!(stop_words::LANGUAGE::Esperanto);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod estonian {
    test!(stop_words::LANGUAGE::Estonian);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod persian {
    test!(stop_words::LANGUAGE::Persian);
}

#[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
#[cfg(test)]
mod finnish {
    test!(stop_words::LANGUAGE::Finnish);
}

#[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
#[cfg(test)]
mod french {
    test!(stop_words::LANGUAGE::French);
}

#[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
#[cfg(test)]
mod german {
    test!(stop_words::LANGUAGE::German);
}

#[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
#[cfg(test)]
mod greek {
    test!(stop_words::LANGUAGE::Greek);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod gujarati {
    test!(stop_words::LANGUAGE::Gujarati);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod hebrew {
    test!(stop_words::LANGUAGE::Hebrew);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod hindi {
    test!(stop_words::LANGUAGE::Hindi);
}

#[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
#[cfg(test)]
mod hungarian {
    test!(stop_words::LANGUAGE::Hungarian);
}

#[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
#[cfg(test)]
mod indonesian {
    test!(stop_words::LANGUAGE::Indonesian);
}

#[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
#[cfg(test)]
mod italian {
    test!(stop_words::LANGUAGE::Italian);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod irish {
    test!(stop_words::LANGUAGE::Irish);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod galician {
    test!(stop_words::LANGUAGE::Galician);
}

#[cfg(feature = "nltk")]
#[cfg(test)]
mod kazakh {
    test!(stop_words::LANGUAGE::Kazakh);
}

#[cfg(feature = "nltk")]
#[cfg(test)]
mod nepali {
    test!(stop_words::LANGUAGE::Nepali);
}

#[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
#[cfg(test)]
mod norwegian {
    test!(stop_words::LANGUAGE::Norwegian);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod polish {
    test!(stop_words::LANGUAGE::Polish);
}

#[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
#[cfg(test)]
mod portuguese {
    test!(stop_words::LANGUAGE::Portuguese);
}

#[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
#[cfg(test)]
mod romanian {
    test!(stop_words::LANGUAGE::Romanian);
}

#[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
#[cfg(test)]
mod russian {
    test!(stop_words::LANGUAGE::Russian);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod slovak {
    test!(stop_words::LANGUAGE::Slovak);
}

#[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
#[cfg(test)]
mod slovenian {
    test!(stop_words::LANGUAGE::Slovenian);
}

#[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
#[cfg(test)]
mod spanish {
    test!(stop_words::LANGUAGE::Spanish);
}

#[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
#[cfg(test)]
mod swedish {
    test!(stop_words::LANGUAGE::Swedish);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod somali {
    test!(stop_words::LANGUAGE::Somali);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod sotho {
    test!(stop_words::LANGUAGE::Sotho);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod swahili {
    test!(stop_words::LANGUAGE::Swahili);
}

#[cfg(feature = "nltk")]
#[cfg(test)]
mod tajik {
    test!(stop_words::LANGUAGE::Tajik);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod thai {
    test!(stop_words::LANGUAGE::Thai);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod tagalog {
    test!(stop_words::LANGUAGE::Tagalog);
}

#[cfg(all(any(feature = "nltk", feature = "iso"), not(feature = "constructed")))]
#[cfg(test)]
mod turkish {
    test!(stop_words::LANGUAGE::Turkish);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod ukrainian {
    test!(stop_words::LANGUAGE::Ukrainian);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod urdu {
    test!(stop_words::LANGUAGE::Urdu);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod vietnamese {
    test!(stop_words::LANGUAGE::Vietnamese);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod yoruba {
    test!(stop_words::LANGUAGE::Yoruba);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod zulu {
    test!(stop_words::LANGUAGE::Zulu);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod hausa {
    test!(stop_words::LANGUAGE::Hausa);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod croatian {
    test!(stop_words::LANGUAGE::Croatian);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod japanese {
    test!(stop_words::LANGUAGE::Japanese);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod korean {
    test!(stop_words::LANGUAGE::Korean);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod kurdish {
    test!(stop_words::LANGUAGE::Kurdish);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod latin {
    test!(stop_words::LANGUAGE::Latin);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod latvian {
    test!(stop_words::LANGUAGE::Latvian);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod lithuanian {
    test!(stop_words::LANGUAGE::Lithuanian);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod marathi {
    test!(stop_words::LANGUAGE::Marathi);
}

#[cfg(all(feature = "iso", not(feature = "nltk"), not(feature = "constructed")))]
#[cfg(test)]
mod malay {
    test!(stop_words::LANGUAGE::Malay);
}

#[cfg(feature = "constructed")]
#[cfg(test)]
mod klingon {
    test!(stop_words::LANGUAGE::Klingon);
}

#[cfg(feature = "constructed")]
#[cfg(test)]
mod dothraki {
    test!(stop_words::LANGUAGE::Dothraki);
}
#[cfg(feature = "constructed")]
#[cfg(test)]
mod dovahzul {
    test!(stop_words::LANGUAGE::Dovahzul);
}

#[cfg(feature = "constructed")]
#[cfg(test)]
mod highvalyrian {
    test!(stop_words::LANGUAGE::HighValyrian);
}

#[cfg(feature = "constructed")]
#[cfg(test)]
mod navi {
    test!(stop_words::LANGUAGE::Navi);
}

#[cfg(feature = "constructed")]
#[cfg(test)]
mod quenya {
    test!(stop_words::LANGUAGE::Quenya);
}

#[cfg(feature = "constructed")]
#[cfg(test)]
mod sindarin {
    test!(stop_words::LANGUAGE::Sindarin);
}
