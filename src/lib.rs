//! This module contains stop words for a number of languages, based on [this resource](https://github.com/Alir3z4/stop-words/tree/bd8cc1434faeb3449735ed570a4a392ab5d35291)

use std::fs;

/// The only function you'll ever need! Given a language code it returns common stop words
pub fn get(language: &str) -> Vec<String> {
    read_from_file("./src/data/".to_owned() + language + ".txt")
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
