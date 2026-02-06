# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - Unreleased

### Added

- `estimate_token_count` — heuristic token count estimation for any text.
- `estimate_token_count_with_options` — estimation with custom language configurations.
- `is_within_token_limit` — check if text fits within a token budget.
- `slice_by_tokens` — extract a substring by estimated token range.
- `split_by_tokens` — split text into chunks of a given token size.
- Built-in language detection for German, French, and Spanish diacritics.
- CJK (Chinese, Japanese, Korean) character-aware counting.
