/// Let's define a macro to help us out
#[cfg(any(feature = "iso", feature = "nltk", feature = "constructed"))]
macro_rules! test {
    (
        $variant:ident
    ) => {
        #[test]
        #[allow(deprecated)] // Verify the original enum remains usable.
        fn compare_enum_to_2letter() {
            // Pull out the name versions that we want
            let lingo = stop_words::Language::$variant;
            let lingo_as_enum = lingo;
            let lingo_as_string: String = lingo.into();
            let lingo_as_str = &*(lingo_as_string.clone());

            // Pull word lists
            let word_list_from_enum = stop_words::get(lingo_as_enum);
            let word_list_from_string = stop_words::get(lingo_as_string);
            let word_list_from_str = stop_words::get(lingo_as_str);

            assert_eq!(
                word_list_from_enum,
                stop_words::get(stop_words::LANGUAGE::$variant)
            );

            // Run a whole hell of a lot of assertions
            for idx in 0..word_list_from_enum.len() {
                assert_eq!(word_list_from_enum[idx], word_list_from_string[idx]);
                assert_eq!(word_list_from_str[idx], word_list_from_string[idx]);
                assert_eq!(word_list_from_enum[idx], word_list_from_str[idx]);
            }
        }

        #[test]
        fn make_sure_list_is_not_empty() {
            let x = stop_words::get(stop_words::Language::$variant);
            assert!(x.len() > 0)
        }
    };
}

#[cfg(any(feature = "nltk", feature = "iso"))]
#[cfg(test)]
mod arabic {
    test!(Arabic);
}

#[cfg(feature = "nltk")]
#[cfg(test)]
mod albanian {
    test!(Albanian);
}

#[cfg(feature = "nltk")]
#[cfg(test)]
mod azerbaijani {
    test!(Azerbaijani);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod afrikaans {
    test!(Afrikaans);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod armenian {
    test!(Armenian);
}

#[cfg(any(feature = "nltk", feature = "iso"))]
#[cfg(test)]
mod basque {
    test!(Basque);
}

#[cfg(feature = "nltk")]
#[cfg(test)]
mod belarusian {
    test!(Belarusian);
}

#[cfg(any(feature = "nltk", feature = "iso"))]
#[cfg(test)]
mod bengali {
    test!(Bengali);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod breton {
    test!(Breton);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod bulgarian {
    test!(Bulgarian);
}

#[cfg(any(feature = "nltk", feature = "iso"))]
#[cfg(test)]
mod catalan {
    test!(Catalan);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod czech {
    test!(Czech);
}

#[cfg(any(feature = "nltk", feature = "iso"))]
#[cfg(test)]
mod chinese {
    test!(Chinese);
}

#[cfg(any(feature = "nltk", feature = "iso"))]
#[cfg(test)]
mod danish {
    test!(Danish);
}

#[cfg(any(feature = "nltk", feature = "iso"))]
#[cfg(test)]
mod dutch {
    test!(Dutch);
}

#[cfg(any(feature = "nltk", feature = "iso"))]
#[cfg(test)]
mod english {
    test!(English);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod esperanto {
    test!(Esperanto);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod estonian {
    test!(Estonian);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod persian {
    test!(Persian);
}

#[cfg(any(feature = "nltk", feature = "iso"))]
#[cfg(test)]
mod finnish {
    test!(Finnish);
}

#[cfg(any(feature = "nltk", feature = "iso"))]
#[cfg(test)]
mod french {
    test!(French);
}

#[cfg(any(feature = "nltk", feature = "iso"))]
#[cfg(test)]
mod german {
    test!(German);
}

#[cfg(any(feature = "nltk", feature = "iso"))]
#[cfg(test)]
mod greek {
    test!(Greek);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod gujarati {
    test!(Gujarati);
}

#[cfg(any(feature = "nltk", feature = "iso"))]
#[cfg(test)]
mod hebrew {
    test!(Hebrew);
}

#[cfg(feature = "nltk")]
#[cfg(test)]
mod hinglish {
    test!(Hinglish);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod hindi {
    test!(Hindi);
}

#[cfg(any(feature = "nltk", feature = "iso"))]
#[cfg(test)]
mod hungarian {
    test!(Hungarian);
}

#[cfg(any(feature = "nltk", feature = "iso"))]
#[cfg(test)]
mod indonesian {
    test!(Indonesian);
}

#[cfg(any(feature = "nltk", feature = "iso"))]
#[cfg(test)]
mod italian {
    test!(Italian);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod irish {
    test!(Irish);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod galician {
    test!(Galician);
}

#[cfg(feature = "nltk")]
#[cfg(test)]
mod kazakh {
    test!(Kazakh);
}

#[cfg(feature = "nltk")]
#[cfg(test)]
mod nepali {
    test!(Nepali);
}

#[cfg(any(feature = "nltk", feature = "iso"))]
#[cfg(test)]
mod norwegian {
    test!(Norwegian);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod polish {
    test!(Polish);
}

#[cfg(any(feature = "nltk", feature = "iso"))]
#[cfg(test)]
mod portuguese {
    test!(Portuguese);
}

#[cfg(any(feature = "nltk", feature = "iso"))]
#[cfg(test)]
mod romanian {
    test!(Romanian);
}

#[cfg(any(feature = "nltk", feature = "iso"))]
#[cfg(test)]
mod russian {
    test!(Russian);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod slovak {
    test!(Slovak);
}

#[cfg(any(feature = "nltk", feature = "iso"))]
#[cfg(test)]
mod slovenian {
    test!(Slovenian);
}

#[cfg(any(feature = "nltk", feature = "iso"))]
#[cfg(test)]
mod spanish {
    test!(Spanish);
}

#[cfg(any(feature = "nltk", feature = "iso"))]
#[cfg(test)]
mod swedish {
    test!(Swedish);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod somali {
    test!(Somali);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod sotho {
    test!(Sotho);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod swahili {
    test!(Swahili);
}

#[cfg(feature = "nltk")]
#[cfg(test)]
mod tajik {
    test!(Tajik);
}

#[cfg(feature = "nltk")]
#[cfg(test)]
mod tamil {
    test!(Tamil);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod thai {
    test!(Thai);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod tagalog {
    test!(Tagalog);
}

#[cfg(any(feature = "nltk", feature = "iso"))]
#[cfg(test)]
mod turkish {
    test!(Turkish);
}

#[cfg(feature = "nltk")]
#[cfg(test)]
mod uzbek {
    test!(Uzbek);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod ukrainian {
    test!(Ukrainian);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod urdu {
    test!(Urdu);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod vietnamese {
    test!(Vietnamese);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod yoruba {
    test!(Yoruba);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod zulu {
    test!(Zulu);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod hausa {
    test!(Hausa);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod croatian {
    test!(Croatian);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod japanese {
    test!(Japanese);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod korean {
    test!(Korean);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod kurdish {
    test!(Kurdish);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod latin {
    test!(Latin);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod latvian {
    test!(Latvian);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod lithuanian {
    test!(Lithuanian);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod marathi {
    test!(Marathi);
}

#[cfg(feature = "iso")]
#[cfg(test)]
mod malay {
    test!(Malay);
}

#[cfg(feature = "constructed")]
#[cfg(test)]
mod klingon {
    test!(Klingon);
}

#[cfg(feature = "constructed")]
#[cfg(test)]
mod dothraki {
    test!(Dothraki);
}
#[cfg(feature = "constructed")]
#[cfg(test)]
mod dovahzul {
    test!(Dovahzul);
}

#[cfg(feature = "constructed")]
#[cfg(test)]
mod highvalyrian {
    test!(HighValyrian);
}

#[cfg(feature = "constructed")]
#[cfg(test)]
mod navi {
    test!(Navi);
}

#[cfg(feature = "constructed")]
#[cfg(test)]
mod quenya {
    test!(Quenya);
}

#[cfg(feature = "constructed")]
#[cfg(test)]
mod sindarin {
    test!(Sindarin);
}
