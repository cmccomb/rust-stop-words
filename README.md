[![Build Status](https://travis-ci.org/cmccomb/stop-words.svg?branch=master)](https://travis-ci.org/cmccomb/stop-words)
# About
This crate contains common stop words for a variety of languages. All stop word lists are from [this resource](https://github.com/Alir3z4/stop-words/tree/bd8cc1434faeb3449735ed570a4a392ab5d35291). This currently includes the following languages:
- Arabic
- Bulgarian
- Catalan
- Czech
- Danish
- Dutch
- English
- Finnish
- French
- German
- Hebrew
- Hindi
- Hungarian
- Indonesian
- Italian
- Norwegian
- Polish
- Portuguese
- Romanian
- Russian
- Slovak
- Spanish
- Swedish
- Turkish
- Ukrainian
- Veetnamese

# Installation
Install through ``crates.io`` with:
```
cargo install stop_words
```


# Example Usage
Using this crate is fairly straight-forward: 
```
use stop_words;

fn main() {
    // Get the stop words
    let words = stop_words::get("english");

    // Print them
    for word in words {
        println!("{}", word)
    }
}
```
The function ``get`` accepts full language names (in English), ISO 693-1 language codes, and ISO 693-2T language codes.