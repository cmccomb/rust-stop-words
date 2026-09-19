#[cfg(any(feature = "iso", feature = "nltk"))]
#[test]
#[allow(deprecated)] // Exercise compatibility with the original public name.
fn legacy_language_preserves_the_existing_api() {
    use stop_words::LANGUAGE::English;

    let language: stop_words::LANGUAGE = English;
    assert_eq!(
        stop_words::get(language),
        stop_words::get(stop_words::Language::English)
    );
    assert_eq!(String::from(language), "en");
    assert_eq!(language.to_string(), "en");
}

#[cfg(any(feature = "iso", feature = "nltk"))]
#[test]
fn lookup_accepts_enum_and_owned_or_borrowed_codes() {
    let code = String::from("en");
    let expected = Some(stop_words::get("en"));
    assert_eq!(stop_words::lookup(stop_words::Language::English), expected);
    assert_eq!(stop_words::lookup("en"), expected);
    assert_eq!(stop_words::lookup(&code), expected);
    assert_eq!(stop_words::lookup(code), expected);
}

#[test]
fn lookup_requires_an_exact_supported_code() {
    for code in ["not-a-language", "", "EN", " en", "en "] {
        assert!(stop_words::lookup(code).is_none());
    }
}

#[cfg(not(feature = "iso"))]
#[test]
fn lookup_returns_none_for_disabled_language_data() {
    assert!(stop_words::lookup("ja").is_none());
    assert!(!stop_words::available_languages().contains(&"ja"));
}

#[cfg(feature = "unimplemented")]
#[test]
fn lookup_handles_enum_variants_without_data() {
    assert!(stop_words::lookup(stop_words::Language::Afar).is_none());
    assert!(!stop_words::available_languages().contains(&"aa"));
}

#[cfg(not(any(feature = "iso", feature = "nltk", feature = "constructed")))]
#[test]
fn builds_without_data_have_no_available_languages() {
    assert!(stop_words::available_languages().is_empty());
    assert!(stop_words::lookup("en").is_none());
}

#[test]
#[should_panic(expected = "The 'not-a-language' language is not recognized.")]
fn legacy_get_still_panics_for_unknown_languages() {
    stop_words::get("not-a-language");
}

#[test]
fn available_languages_are_sorted_and_resolvable() {
    let languages = stop_words::available_languages();
    assert!(languages.windows(2).all(|pair| pair[0] < pair[1]));
    assert!(languages
        .iter()
        .all(|language| stop_words::lookup(language).is_some()));
}

#[cfg(all(feature = "iso", feature = "nltk", feature = "constructed"))]
#[test]
fn all_language_sources_coexist() {
    for language in [
        stop_words::Language::Japanese,
        stop_words::Language::Tamil,
        stop_words::Language::Klingon,
    ] {
        assert!(!stop_words::lookup(language).unwrap().is_empty());
    }
}
