// // use stop_words;
//
// /// Let's define a macro to help us out
// macro_rules! test {
//     (
//         $language_full:literal,
//         $language_2:literal,
//         $language_3:literal,
//         $nltk_panic:ident
//     ) => {
//         #[test]
//         fn compare_full_to_2letter() {
//             let x = stop_words::get($language_full);
//             let y = stop_words::get($language_2);
//             for idx in 0..x.len() {
//                 assert_eq!(x[idx], y[idx])
//             }
//         }
//
//         #[test]
//         fn compare_full_to_3letter() {
//             let x = stop_words::get($language_full);
//             let y = stop_words::get($language_3);
//             for idx in 0..x.len() {
//                 assert_eq!(x[idx], y[idx])
//             }
//         }
//
//         #[test]
//         #[$nltk_panic]
//         fn check_nltk() {
//             let x = stop_words::get_nltk($language_full);
//             for idx in 0..x.len() {
//                 println!("{}", x[idx])
//             }
//         }
//     };
// }
//
// #[cfg(test)]
// mod arabic {
//     test!("arabic", "ar", "ara", test);
// }
//
// #[cfg(test)]
// mod azerbaijani {
//     test!("azerbaijani", "az", "aze", test);
// }
//
// #[cfg(test)]
// mod bulgarian {
//     test!("bulgarian", "bg", "bul", should_panic);
// }
//
// #[cfg(test)]
// mod catalan {
//     test!("catalan", "ca", "cat", should_panic);
// }
//
// #[cfg(test)]
// mod czech {
//     test!("czech", "cs", "ces", should_panic);
// }
//
// #[cfg(test)]
// mod danish {
//     test!("danish", "da", "dan", test);
// }
//
// #[cfg(test)]
// mod dutch {
//     test!("dutch", "nl", "nld", test);
// }
//
// #[cfg(test)]
// mod english {
//     test!("english", "en", "eng", test);
// }
//
// #[cfg(test)]
// mod finnish {
//     test!("finnish", "fi", "fin", test);
// }
//
// #[cfg(test)]
// mod french {
//     test!("french", "fr", "fra", test);
// }
//
// #[cfg(test)]
// mod german {
//     test!("german", "de", "deu", test);
// }
//
// #[cfg(test)]
// mod greek {
//     test!("greek", "el", "ell", test);
// }
//
// #[cfg(test)]
// mod hebrew {
//     test!("hebrew", "he", "heb", should_panic);
// }
//
// #[cfg(test)]
// mod hindi {
//     test!("hindi", "hi", "hin", should_panic);
// }
//
// #[cfg(test)]
// mod hungarian {
//     test!("hungarian", "hu", "hun", test);
// }
//
// #[cfg(test)]
// mod indonesian {
//     test!("indonesian", "in", "ind", test);
// }
//
// #[cfg(test)]
// mod italian {
//     test!("italian", "it", "ita", test);
// }
//
// #[cfg(test)]
// mod kazakh {
//     test!("kazakh", "kk", "kaz", test);
// }
//
// #[cfg(test)]
// mod nepali {
//     test!("nepali", "ne", "nep", test);
// }
//
// #[cfg(test)]
// mod norwegian {
//     test!("norwegian", "nn", "nno", test);
// }
//
// #[cfg(test)]
// mod polish {
//     test!("polish", "pl", "pol", should_panic);
// }
//
// #[cfg(test)]
// mod portuguese {
//     test!("portuguese", "pt", "por", test);
// }
//
// #[cfg(test)]
// mod romanian {
//     test!("romanian", "ro", "ron", test);
// }
//
// #[cfg(test)]
// mod russian {
//     test!("russian", "ru", "rus", test);
// }
//
// #[cfg(test)]
// mod slovak {
//     test!("slovak", "sk", "slk", should_panic);
// }
//
// #[cfg(test)]
// mod slovenian {
//     test!("slovenian", "sl", "slv", test);
// }
//
// #[cfg(test)]
// mod spanish {
//     test!("spanish", "es", "spa", test);
// }
//
// #[cfg(test)]
// mod swedish {
//     test!("swedish", "sv", "swe", test);
// }
//
// #[cfg(test)]
// mod tajik {
//     test!("tajik", "tg", "tgk", test);
// }
//
// #[cfg(test)]
// mod turkish {
//     test!("turkish", "tr", "tur", test);
// }
//
// #[cfg(test)]
// mod ukrainian {
//     test!("ukrainian", "uk", "ukr", should_panic);
// }
//
// #[cfg(test)]
// mod vietnamese {
//     test!("vietnamese", "vi", "vie", should_panic);
// }
