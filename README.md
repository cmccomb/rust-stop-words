[![Build Status](https://travis-ci.org/cmccomb/stop-words.svg?branch=master)](https://travis-ci.org/cmccomb/stop-words)
# About
This crate contains common stop words for a variety of languages. All stop word lists are from [this resource](https://github.com/Alir3z4/stop-words/tree/bd8cc1434faeb3449735ed570a4a392ab5d35291)

# Example Usage
Using this crate is fairly straight-forward: 
```
use stop_words;

fn main() {
    let words = stop_words::get("english");
    for word in words {
        println!("{}", word)
    }
}
```