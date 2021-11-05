//! Helper functions

// Strum contains all the trait definitions
use super::language_names::LANGUAGE;

// /// This function converts a language code to a full language name
// pub(crate) fn get_language_from_iso(code: &str) -> &str {
//     let mut iter = ISO_LANGUAGES.iter();
//     let idx = iter.position(|&x| x == code);
//     match idx {
//         Some(x) => ISO_LANGUAGES[x],
//         None => panic!(
//             "The {} language code is nonexistent or not currently supported.",
//             code
//         ),
//     }
// }
