# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [0.10.1] - 2026-09-19

### Added

- `Language` as the preferred enum, retaining the original `LANGUAGE` enum for compatibility.
- `lookup` for non-panicking language lookup, accepting enums and string codes.
- `available_languages` for runtime discovery of enabled language codes.
- The convenience feature `all`, enabling ISO, NLTK, and constructed lists.

### Deprecated

- `LANGUAGE` in favor of `Language`; existing enum and variant imports continue to work with a deprecation warning. Both enums accept the same lookup codes and are generated from one definition.

### Fixed

- Feature combinations are now additive; constructed and NLTK features no longer hide unrelated enum variants.
- Builds with no language-data features enabled compile successfully.
- Removed a duplicate entry from the Klingon list.
- Corrected repository and homepage metadata.
- Included the Apache-2.0 license text advertised by the crate's dual-license metadata.

### Changed

- Retained the Rust 2021 edition and the existing `get` API.
- Preserved the published ISO/NLTK lists and NLTK precedence when both sources are enabled.
- Fictional constructed-language lists are documented as experimental and retain their original provenance.

[Unreleased]: https://github.com/cmccomb/rust-stop-words/compare/v0.10.1...HEAD
[0.10.1]: https://github.com/cmccomb/rust-stop-words/compare/v0.10.0...v0.10.1
