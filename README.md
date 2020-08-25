[![Build Status](https://travis-ci.org/cmccomb/rust-stop-words.svg?branch=master)](https://travis-ci.org/cmccomb/rust-stop-words)
[![Crates.io](https://img.shields.io/crates/v/stop-words.svg)](https://crates.io/crates/stop-words)
[![docs.rs](https://docs.rs/stop-words/badge.svg)](https://docs.rs/stop-words)
# About
Stop words are words that don't carry much meaning, and are typically removed as a preprocessing step before text
analysis or natural language processing. This crate contains common stop words for a variety of languages. This crate uses stop word
lists from [this resource](https://github.com/Alir3z4/stop-words/tree/bd8cc1434faeb3449735ed570a4a392ab5d35291) and also from [NLTK](https://www.nltk.org/).

This crate currently includes the following languages:
- Arabic
- Azerbaijani
- Bulgarian
- Catalan
- Czech
- Danish
- Dutch
- English
- Finnish
- French
- German
- Greek
- Hebrew
- Hindi
- Hungarian
- Indonesian
- Kazakh
- Italian
- Nepali
- Norwegian
- Polish
- Portuguese
- Romanian
- Russian
- Slovak
- Slovenian
- Spanish
- Swedish
- Tajik
- Turkish
- Ukrainian
- Vietnamese

# Installation
Install through ``crates.io`` with:
```shell script
cargo install stop_words
```

Then add it to your ``Cargo.toml` with:
```toml
[dependencies]
stop-words = "0.2.1"
```
and add this to your root:
```rust
use stop_words;
```

# Usage
Using this crate is fairly straight-forward: 
```rust
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
The function ``get`` will pull stop words in all of the languages given above, drawing on 
[this resource](https://github.com/Alir3z4/stop-words/tree/bd8cc1434faeb3449735ed570a4a392ab5d35291) and also from 
[NLTK](https://www.nltk.org/) if the target language doesn't exist in the former. If you'd like to specifically get stop
words from NLTK, that's easy too, just do:
```
let words = stop_words::get_nltk("en");
```


Both ``get`` and ``get_nltk`` accept full language names (in English), ISO 693-1 language codes (2-letter codes), and 
ISO 693-2T (3-letter codes) language codes. This means you can also do this:
```rust
let words = stop_words::get("en");
```
or this:
```rust
let words = stop_words::get("eng");
```


Finally, you can convert the ``Vec<String>`` of words to a ``HashSet<String>``. I'm not here to judge you.
```rust
let vec = stop_words::get("en");
let set = stop_words::vec_to_set(vec);
```


