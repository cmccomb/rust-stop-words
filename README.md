[![Github CI](https://github.com/cmccomb/rust-stop-words/actions/workflows/tests.yml/badge.svg)](https://github.com/cmccomb/rust-stop-words/actions)
[![Crates.io](https://img.shields.io/crates/v/stop-words.svg)](https://crates.io/crates/stop-words)
[![docs.rs](https://img.shields.io/docsrs/stop-words/latest?logo=rust)](https://docs.rs/stop-words)
# About

Stop words are words that don't carry much meaning, and are typically removed as a preprocessing step before text
analysis or natural language processing. This crate contains common stop words for a variety of languages. This crate uses stop word
lists from [Stopwords ISO](https://github.com/stopwords-iso) and also from [NLTK](https://www.nltk.org/).

# Usage
Using this crate is fairly straight-forward: 
```rust, ignore
// Get the stop words
let words = stop_words::get(stop_words::LANGUAGE::English);

// Print them
for word in words {
    println!("{}", word);
}
```
The function ``get`` will take either a member of the `LANGUAGE` enum or a two-letter ISO language code as either a `str` or a `String` type.

You can find a complete example of how to read in a text file and remove stop words [here](https://github.com/cmccomb/rust-stop-words/blob/master/examples/remove_stop_words_with_regex.rs).


# ISO Language Availability
This crate supports all languages from [Stopwords ISO](https://github.com/stopwords-iso) and also from [NLTK](https://www.nltk.org/). Expand the table below to see a comprehensive description.
<details>
    <summary>Language Coverage Table</summary>

| ISO 639-1 Code | Language                                                                         | Stopwords ISO | NLTK |
|----------------|----------------------------------------------------------------------------------|---------------|------|
| aa             | Afar                                                                             |               |      |
| ab             | Abkhazian                                                                        |               |      |
| af             | Afrikaans                                                                        | ✓             |      |
| ak             | Akan                                                                             |               |      |
| sq             | Albanian                                                                         |               |      |
| am             | Amharic                                                                          |               |      |
| ar             | Arabic                                                                           | ✓             | ✓    |
| an             | Aragonese                                                                        |               |      |
| hy             | Armenian                                                                         | ✓             |      |
| as             | Assamese                                                                         |               |      |
| av             | Avaric                                                                           |               |      |
| ae             | Avestan                                                                          |               |      |
| ay             | Aymara                                                                           |               |      |
| az             | Azerbaijani                                                                      |               | ✓    |
| ba             | Bashkir                                                                          |               |      |
| bm             | Bambara                                                                          |               |      |
| eu             | Basque                                                                           | ✓             |      |
| be             | Belarusian                                                                       |               |      |
| bn             | Bengali                                                                          | ✓             |      |
| bh             | Bihari languages                                                                 |               |      |
| bi             | Bislama                                                                          |               |      |
| bo             | Tibetan                                                                          |               |      |
| bs             | Bosnian                                                                          |               |      |
| br             | Breton                                                                           | ✓             |      |
| bg             | Bulgarian                                                                        | ✓             |      |
| my             | Burmese                                                                          |               |      |
| ca             | Catalan; Valencian                                                               | ✓             |      |
| cs             | Czech                                                                            | ✓             |      |
| ch             | Chamorro                                                                         |               |      |
| ce             | Chechen                                                                          |               |      |
| zh             | Chinese                                                                          | ✓             |      |
| cu             | Church Slavic; Old Slavonic; Church Slavonic; Old Bulgarian; Old Church Slavonic |               |      |
| cv             | Chuvash                                                                          |               |      |
| kw             | Cornish                                                                          |               |      |
| co             | Corsican                                                                         |               |      |
| cr             | Cree                                                                             |               |      |
| cy             | Welsh                                                                            |               |      |
| da             | Danish                                                                           | ✓             | ✓    |
| de             | German                                                                           | ✓             | ✓    |
| dv             | Divehi; Dhivehi; Maldivian                                                       |               |      |
| nl             | Dutch; Flemish                                                                   | ✓             | ✓    |
| dz             | Dzongkha                                                                         |               |      |
| el             | Greek, Modern (1453-)                                                            | ✓             | ✓    |
| en             | English                                                                          | ✓             | ✓    |
| eo             | Esperanto                                                                        | ✓             |      |
| et             | Estonian                                                                         | ✓             |      |
| ee             | Ewe                                                                              |               |      |
| fo             | Faroese                                                                          |               |      |
| fa             | Persian                                                                          | ✓             |      |
| fj             | Fijian                                                                           |               |      |
| fi             | Finnish                                                                          | ✓             | ✓    |
| fr             | French                                                                           | ✓             | ✓    |
| fy             | Western Frisian                                                                  |               |      |
| ff             | Fulah                                                                            |               |      |
| ka             | Georgian                                                                         |               |      |
| gd             | Gaelic; Scottish Gaelic                                                          |               |      |
| ga             | Irish                                                                            | ✓             |      |
| gl             | Galician                                                                         | ✓             |      |
| gv             | Manx                                                                             |               |      |
| gn             | Guarani                                                                          |               |      |
| gu             | Gujarati                                                                         | ✓             |      |
| ht             | Haitian; Haitian Creole                                                          |               |      |
| ha             | Hausa                                                                            | ✓             |      |
| he             | Hebrew                                                                           | ✓             |      |
| hz             | Herero                                                                           |               |      |
| hi             | Hindi                                                                            | ✓             |      |
| ho             | Hiri Motu                                                                        |               |      |
| hr             | Croatian                                                                         | ✓             |      |
| hu             | Hungarian                                                                        | ✓             | ✓    |
| ig             | Igbo                                                                             |               |      |
| is             | Icelandic                                                                        |               |      |
| io             | Ido                                                                              |               |      |
| ii             | Sichuan Yi; Nuosu                                                                |               |      |
| iu             | Inuktitut                                                                        |               |      |
| ie             | Interlingue; Occidental                                                          |               |      |
| ia             | Interlingua (International Auxiliary Language Association)                       |               |      |
| id             | Indonesian                                                                       | ✓             | ✓    |
| ik             | Inupiaq                                                                          |               |      |  
| it             | Italian                                                                          | ✓             | ✓    |
| jv             | Javanese                                                                         |               |      |
| ja             | Japanese                                                                         | ✓             |      |
| kl             | Kalaallisut; Greenlandic                                                         |               |      |
| kn             | Kannada                                                                          |               |      |
| ks             | Kashmiri                                                                         |               |      |
| kr             | Kanuri                                                                           |               |      |
| kk             | Kazakh                                                                           |               | ✓    |
| km             | Central Khmer                                                                    |               |      |
| ki             | Kikuyu; Gikuyu                                                                   |               |      |
| rw             | Kinyarwanda                                                                      |               |      |
| ky             | Kirghiz; Kyrgyz                                                                  |               |      |
| kv             | Komi                                                                             |               |      |
| kg             | Kongo                                                                            |               |      |
| ko             | Korean                                                                           | ✓             |      |
| kj             | Kuanyama; Kwanyama                                                               |               |      |
| ku             | Kurdish                                                                          | ✓             |      |
| lo             | Lao                                                                              |               |      |
| la             | Latin                                                                            | ✓             |      |
| lv             | Latvian                                                                          | ✓             |      |
| li             | Limburgan; Limburger; Limburgish                                                 |               |      |
| ln             | Lingala                                                                          |               |      |
| lt             | Lithuanian                                                                       | ✓             |      |
| lb             | Luxembourgish; Letzeburgesch                                                     |               |      |
| lu             | Luba-Katanga                                                                     |               |      |
| lg             | Ganda                                                                            |               |      |
| mk             | Macedonian                                                                       |               |      |
| mh             | Marshallese                                                                      |               |      |
| ml             | Malayalam                                                                        |               |      |
| mi             | Maori                                                                            |               |      |
| mr             | Marathi                                                                          | ✓             |      |
| ms             | Malay                                                                            | ✓             |      |
| mg             | Malagasy                                                                         |               |      |
| mt             | Maltese                                                                          |               |      |
| mn             | Mongolian                                                                        |               |      |
| na             | Nauru                                                                            |               |      |
| nv             | Navajo; Navaho                                                                   |               |      |
| nr             | Ndebele, South; South Ndebele                                                    |               |      |
| nd             | Ndebele, North; North Ndebele                                                    |               |      |
| ng             | Ndonga                                                                           |               |      |
| ne             | Nepali                                                                           |               | ✓    |
| nn             | Norwegian Nynorsk; Nynorsk, Norwegian                                            |               |      |
| nb             | Bokmål, Norwegian; Norwegian Bokmål                                              |               |      |
| no             | Norwegian                                                                        | ✓             | ✓    |
| ny             | Chichewa; Chewa; Nyanja                                                          |               |      |
| oc             | Occitan (post 1500)                                                              |               |      |
| oj             | Ojibwa                                                                           |               |      |
| or             | Oriya                                                                            |               |      |
| om             | Oromo                                                                            |               |      |
| os             | Ossetian; Ossetic                                                                |               |      |
| pa             | Panjabi; Punjabi                                                                 |               |      |
| pi             | Pali                                                                             |               |      |
| pl             | Polish                                                                           | ✓             |      |
| pt             | Portuguese                                                                       | ✓             | ✓    |
| ps             | Pushto; Pashto                                                                   |               |      |
| qu             | Quechua                                                                          |               |      |
| rm             | Romansh                                                                          |               |      |
| ro             | Romanian; Moldavian; Moldovan                                                    | ✓             | ✓    |
| rn             | Rundi                                                                            |               |      |
| ru             | Russian                                                                          | ✓             | ✓    |
| sg             | Sango                                                                            |               |      |
| sa             | Sanskrit                                                                         |               |      |
| si             | Sinhala; Sinhalese                                                               |               |      |
| sk             | Slovak                                                                           | ✓             |      |
| sl             | Slovenian                                                                        | ✓             | ✓    |
| se             | Northern Sami                                                                    |               |      |
| sm             | Samoan                                                                           |               |      |
| sn             | Shona                                                                            |               |      |
| sd             | Sindhi                                                                           |               |      |
| so             | Somali                                                                           | ✓             |      |
| st             | Sotho, Southern                                                                  | ✓             |      |
| es             | Spanish; Castilian                                                               | ✓             | ✓    |
| sc             | Sardinian                                                                        |               |      |
| sr             | Serbian                                                                          |               |      |
| ss             | Swati                                                                            |               |      |
| su             | Sundanese                                                                        |               |      |
| sw             | Swahili                                                                          | ✓             |      |
| sv             | Swedish                                                                          | ✓             | ✓    |
| ty             | Tahitian                                                                         |               |      |
| ta             | Tamil                                                                            |               |      |
| tt             | Tatar                                                                            |               |      |
| te             | Telugu                                                                           |               |      |
| tg             | Tajik                                                                            |               | ✓    |
| tl             | Tagalog                                                                          | ✓             |      |
| th             | Thai                                                                             | ✓             |      |
| ti             | Tigrinya                                                                         |               |      |
| to             | Tonga (Tonga Islands)                                                            |               |      |
| tn             | Tswana                                                                           |               |      |
| ts             | Tsonga                                                                           |               |      |
| tk             | Turkmen                                                                          |               |      |
| tr             | Turkish                                                                          | ✓             | ✓    |
| tw             | Twi                                                                              |               |      |
| ug             | Uighur; Uyghur                                                                   |               |      |
| uk             | Ukrainian                                                                        | ✓             |      |
| ur             | Urdu                                                                             | ✓             |      |
| uz             | Uzbek                                                                            |               |      |
| ve             | Venda                                                                            |               |      |
| vi             | Vietnamese                                                                       | ✓             |      |
| vo             | Volapük                                                                          |               |      |
| wa             | Walloon                                                                          |               |      |
| wo             | Wolof                                                                            |               |      |
| xh             | Xhosa                                                                            |               |      |
| yi             | Yiddish                                                                          |               |      |
| yo             | Yoruba                                                                           | ✓             |      |
| za             | Zhuang; Chuang                                                                   |               |      |
| zu             | Zulu                                                                             | ✓             |      |

</details>

# Constructed Language Availability
We also support some constructed (fictional/fantasy) languages! Expand the table below to see a comprehensive description. ChatGPT was used to generate these lists quickly, so they are incomplete and approximate. Help welcome! To use these languages, add the `constructed` feature.

<details>
    <summary>Language Coverage Table</summary>

| ISO 639-3 Code           | Language                                                                                    |
|--------------------------|---------------------------------------------------------------------------------------------|
| qya                      | [Quenya](https://en.wikipedia.org/wiki/Quenya)                                              |
| sjn                      | [Sindarin](https://en.wikipedia.org/wiki/Sindarin)                                          |
| tlh                      | [Klingon](https://en.wikipedia.org/wiki/Klingon)                                            |
| mis (_dot_ is used here) | [Dothraki](https://en.wikipedia.org/wiki/Dothraki_language)                                 |
| mis (_dov_ is used here) | [Dovahzul](https://www.thuum.org/library/Dovahzul%20Print%20Dictionary%204th%20Edition.pdf) |
| mis (_nav_ is used here) | [Navi](https://en.wikipedia.org/wiki/Na%CA%BCvi_language)                                   | 
| mis (_val_ is used here) | [High Valyrian](https://en.wikipedia.org/wiki/Valyrian_languages)                           |

The following prompt was used with the Mar 14, 2023 version of ChatGPT:
```text
Please give me one list of 20+ stop words for each of the following languages: Sindarin, Quenya, Dothraki, Na'vi, 
Dovahzul, Klingon, and High Valyrian. I'd like the lists to be formatted as follows:
Sindarin
1. [word goes here]
2. [word goes here]
...
Quenya
1. [word goes here]
...

And so on.
```

</details>
