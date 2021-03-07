// use stop_words;

/// Let's define a macro to help us out
macro_rules! test {
    (
        $language_full:literal,
        $language_2:literal
    ) => {
        use std::str::FromStr;

        #[test]
        fn compare_enum_to_2letter() {
            let lingo = stop_words::LANGUAGE::from_str($language_full).unwrap();
            println!("{:?}", lingo);
            let x = stop_words::get(lingo);
            println!("{:?}", x);
            let y = stop_words::get_iso($language_2);
            println!("{:?}", y);
            for idx in 0..x.len() {
                assert_eq!(x[idx], y[idx])
            }
        }
    };
}

#[cfg(test)]
mod arabic {
    test!("Arabic", "ar");
}

#[cfg(feature = "nltk")]
#[cfg(test)]
mod azerbaijani {
    test!("Azerbaijani", "az");
}

#[cfg(not(feature = "nltk"))]
#[cfg(test)]
mod bulgarian {
    test!("Bulgarian", "bg");
}

#[cfg(not(feature = "nltk"))]
#[cfg(test)]
mod catalan {
    test!("Catalan", "ca");
}

#[cfg(not(feature = "nltk"))]
#[cfg(test)]
mod czech {
    test!("Czech", "cs");
}

#[cfg(test)]
mod danish {
    test!("Danish", "da");
}

#[cfg(test)]
mod dutch {
    test!("Dutch", "nl");
}

#[cfg(test)]
mod english {
    test!("English", "en");
}

#[cfg(test)]
mod finnish {
    test!("Finnish", "fi");
}

#[cfg(test)]
mod french {
    test!("French", "fr");
}

#[cfg(test)]
mod german {
    test!("German", "de");
}

#[cfg(test)]
mod greek {
    test!("Greek", "el");
}

#[cfg(not(feature = "nltk"))]
#[cfg(test)]
mod gujarati {
    test!("Gujarati", "gu");
}

#[cfg(not(feature = "nltk"))]
#[cfg(test)]
mod hebrew {
    test!("Hebrew", "he");
}

#[cfg(not(feature = "nltk"))]
#[cfg(test)]
mod hindi {
    test!("Hindi", "hi");
}

#[cfg(test)]
mod hungarian {
    test!("Hungarian", "hu");
}

#[cfg(test)]
mod indonesian {
    test!("Indonesian", "id");
}

#[cfg(test)]
mod italian {
    test!("Italian", "it");
}

#[cfg(feature = "nltk")]
#[cfg(test)]
mod kazakh {
    test!("Kazakh", "kk");
}

#[cfg(feature = "nltk")]
#[cfg(test)]
mod nepali {
    test!("Nepali", "ne");
}

#[cfg(test)]
mod norwegian {
    test!("Norwegian", "no");
}

#[cfg(not(feature = "nltk"))]
#[cfg(test)]
mod polish {
    test!("Polish", "pl");
}

#[cfg(test)]
mod portuguese {
    test!("Portuguese", "pt");
}

#[cfg(test)]
mod romanian {
    test!("Romanian", "ro");
}

#[cfg(test)]
mod russian {
    test!("Russian", "ru");
}

#[cfg(not(feature = "nltk"))]
#[cfg(test)]
mod slovak {
    test!("Slovak", "sk");
}

#[cfg(test)]
mod slovenian {
    test!("Slovenian", "sl");
}

#[cfg(test)]
mod spanish {
    test!("Spanish", "es");
}

#[cfg(test)]
mod swedish {
    test!("Swedish", "sv");
}

#[cfg(feature = "nltk")]
#[cfg(test)]
mod tajik {
    test!("Tajik", "tg");
}

#[cfg(test)]
mod turkish {
    test!("Turkish", "tr");
}

#[cfg(not(feature = "nltk"))]
#[cfg(test)]
mod ukrainian {
    test!("Ukrainian", "uk");
}

#[cfg(not(feature = "nltk"))]
#[cfg(test)]
mod vietnamese {
    test!("Vietnamese", "vi");
}
