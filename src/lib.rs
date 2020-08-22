//! This module contains stop words for a number of languages, based on [this resource](https://github.com/Alir3z4/stop-words/tree/bd8cc1434faeb3449735ed570a4a392ab5d35291)

use std::fs;

const LANGUAGES: [&str; 26] = ["arabic", "catalan", "danish", "english", "french", "hindi",
    "indonesian", "languages", "norwegian", "portuguese", "russian", "spanish", "turkish",
    "vietnamese", "bulgarian", "czech", "dutch", "finnish", "german", "hungarian", "italian",
    "polish", "romanian", "slovak", "swedish", "ukrainian"];

// TODO: Make sure filepaths work across platforms
// TODO: Better error handling here - maybe use Result?
/// The only function you'll ever need! Given a language code it returns common stop words
pub fn get(language: &str) -> Vec<String> {
    if language_exists(language) {
        read_from_file("./src/data/".to_owned() + language + ".txt")
    } else {
        vec!["Language not found".to_string(); 0]
    }
}


fn language_exists(language: &str) -> bool {
    let mut language_exists = false;
    for language_option in LANGUAGES.iter() {
        println!("{}", language == *language_option );
        if language == *language_option {
            language_exists = true;
            break;
        }
    }
    language_exists
}

/// This function is only used internally to read in stopwords from file. It is not public.
fn read_from_file(filename: String) -> Vec<String> {

    let contents = fs::read_to_string(fs::canonicalize(filename).unwrap()).unwrap();
    let split_contents = contents.split("\n");
    let mut output = vec![];
    for word in split_contents {
        output.push(String::from(word));
    }
    output
}


// TODO: More tests
#[cfg(test)]
mod tests {
    use crate::get;

    #[test]
    fn it_works() {
        let x = get("english");
        for y in x {
            println!("{}", y);
        }
    }
}
