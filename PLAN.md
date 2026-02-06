# tokenx-rs: Rust Port of tokenx

Fast token count estimation for LLMs at 96% accuracy without a full tokenizer.

## Overview

This document outlines the plan to create `tokenx-rs`, a Rust port of the [tokenx](https://github.com/johannschopplich/tokenx) TypeScript library. The crate will be published under the `qbit-ai` GitHub organization and made available on crates.io.

### Why This Exists

- **Problem**: Accurate token counting requires full BPE tokenizers (tiktoken, etc.) which add 2-4MB of vocabulary files
- **Solution**: Heuristic-based estimation that achieves ~96% accuracy with zero vocabulary overhead
- **Use Case**: Real-time token streaming display, context budget estimation, pre-flight checks

### Accuracy Benchmarks (from original tokenx)

| Content | Actual Tokens | Estimated | Deviation |
|---------|---------------|-----------|-----------|
| Short English text | 19 | 19 | 0.00% |
| German text with umlauts | 48 | 49 | 2.08% |
| Kafka - Metamorphosis (English) | 31,796 | 32,325 | 1.66% |
| Kafka - Die Verwandlung (German) | 35,309 | 33,970 | 3.79% |
| 道德經 - Laozi (Chinese) | 11,712 | 11,427 | 2.43% |
| 羅生門 - Akutagawa (Japanese) | 9,517 | 10,535 | 10.70% |
| TypeScript ES5 declarations (~4000 loc) | 49,293 | 51,599 | 4.68% |

---

## Project Details

| Field | Value |
|-------|-------|
| **Crate Name** | `tokenx-rs` |
| **Repository** | `github.com/qbit-ai/tokenx-rs` |
| **License** | MIT |
| **MSRV** | 1.70 |
| **Initial Version** | 0.1.0 |

---

## Checklist

### Phase 1: Project Setup

- [x] Create GitHub repository `qbit-ai/tokenx-rs`
  - [x] Initialize with README, LICENSE (MIT), .gitignore
  - [x] Set repository description: "Rust port of johannschopplich/tokenx - Fast token count estimation for LLMs at 96% accuracy without a full tokenizer"
  - [x] Add topics: `rust`, `llm`, `tokenizer`, `tokens`, `gpt`, `claude`, `nlp`
  - [x] Enable Issues and Discussions

- [x] Initialize Rust project structure
  - [x] Run `cargo init --lib`
  - [x] Configure `Cargo.toml` with metadata
  - [x] Create directory structure (see below)
  - [x] Add `.rustfmt.toml` for formatting preferences

- [x] Set up GitHub Actions CI/CD
  - [x] `.github/workflows/ci.yml` - Test, clippy, rustfmt on PRs
  - [x] `.github/workflows/publish.yml` - Publish to crates.io on release tag

### Phase 2: Core Implementation

- [x] ~~Implement pattern matching (`src/patterns.rs`)~~ — Replaced with inline char-level classification (no regex)

- [x] Implement configuration types (`src/config.rs`)
  - [x] `LanguageConfig` struct (matcher fn, chars_per_token)
  - [x] `EstimationOptions` struct (default_chars_per_token, language_configs)
  - [x] `SplitOptions` struct (extends EstimationOptions with overlap)
  - [x] Default language configurations (German, French, Spanish)

- [x] Implement core estimator (`src/estimator.rs`)
  - [x] `estimate_token_count(text: &str) -> usize`
  - [x] `estimate_token_count_with_options(text: &str, options: &EstimationOptions) -> usize`
  - [x] Single-pass char scanner with segment scoring (replaced regex split + classify)
  - [x] `detect_language_cpt()` internal function

- [x] Implement utility functions (`src/utils.rs`)
  - [x] `is_within_token_limit(text: &str, limit: usize) -> bool`
  - [x] `slice_by_tokens(text: &str, start: usize, end: Option<usize>) -> String`
  - [x] `split_by_tokens(text: &str, tokens_per_chunk: usize) -> Vec<String>`
  - [x] Support negative indices in slice_by_tokens (like Python slicing)
  - [x] Support overlap in split_by_tokens

- [x] Create public API (`src/lib.rs`)
  - [x] Re-export all public types and functions
  - [x] Module documentation with examples

### Phase 3: Testing

- [x] Unit tests (`src/*.rs` inline tests)
  - [x] Empty string handling
  - [x] Pure whitespace
  - [x] Pure CJK text
  - [x] Pure punctuation
  - [x] Mixed content
  - [x] Numeric strings
  - [x] Short words (≤3 chars)
  - [x] Language-specific text (German, French)
  - [x] Underscore identifiers
  - [x] Custom options

- [x] Integration tests (`tests/accuracy.rs`)
  - [x] Port tokenx benchmark cases
  - [x] Add test fixtures for known texts
  - [x] Document accuracy per test case

- [x] Property-based tests (`tests/proptest.rs`)
  - [x] Arbitrary string input doesn't panic
  - [x] Result is always >= 0
  - [x] Empty input returns 0
  - [x] slice_by_tokens round-trip properties
  - [x] split_by_tokens concatenation properties

- [x] Benchmarks (`benches/estimation.rs`)
  - [x] Short text (~20 tokens)
  - [x] Medium text (~1000 tokens)
  - [x] Long text (~30000 tokens)
  - [x] CJK text
  - [x] Code

### Phase 4: Documentation

- [x] README.md
  - [x] Badges (crates.io version, docs.rs, license)
  - [x] One-line description
  - [x] Credit to original tokenx project
  - [x] Installation instructions
  - [x] Quick start example
  - [x] How it works section
  - [x] Performance benchmarks table (Rust vs Node.js)
  - [x] Accuracy benchmarks table
  - [x] License section

- [x] Rustdoc documentation
  - [x] Crate-level documentation with examples
  - [x] All public functions documented
  - [x] All public types documented
  - [x] Examples for each major function

- [x] CHANGELOG.md
  - [x] Follow Keep a Changelog format
  - [x] Document v0.1.0 initial release

- [x] LICENSE
  - [x] MIT license text

### Phase 5: Publishing

- [x] Pre-publish validation
  - [x] Run `cargo fmt --check`
  - [x] Run `cargo clippy -- -D warnings`
  - [x] Run `cargo test`
  - [x] Run `cargo doc --no-deps`
  - [x] Run `cargo publish --dry-run`
  - [x] Verify all metadata in Cargo.toml

- [ ] Publish to crates.io
  - [ ] Ensure crates.io API token is configured
  - [ ] Run `cargo publish`
  - [ ] Verify crate appears on crates.io
  - [ ] Verify docs appear on docs.rs

- [ ] Create GitHub release
  - [ ] Tag `v0.1.0`
  - [ ] Write release notes
  - [ ] Link to crates.io
  - [ ] Link to docs.rs

### Phase 6: Integration with Qbit

- [ ] Add `tokenx-rs` dependency to `qbit-context/Cargo.toml`
- [ ] Replace `estimate_tokens()` in `token_budget.rs` with `tokenx_rs::estimate_token_count()`
- [ ] Update any related tests
- [ ] Verify streaming token display works correctly

---

## Directory Structure

```
tokenx-rs/
├── .github/
│   └── workflows/
│       ├── ci.yml                 # CI: test, clippy, fmt
│       └── publish.yml            # Publish on release
├── benches/
│   └── estimation.rs              # Criterion benchmarks
├── src/
│   ├── lib.rs                     # Public API, crate docs
│   ├── config.rs                  # LanguageConfig, EstimationOptions
│   ├── estimator.rs               # Core estimation logic (single-pass scanner)
│   └── utils.rs                   # is_within_limit, slice, split
├── tests/
│   ├── accuracy.rs                # Accuracy validation tests
│   └── proptest.rs                # Property-based tests
├── .gitignore
├── .rustfmt.toml
├── Cargo.toml
├── CHANGELOG.md
├── LICENSE
└── README.md
```

---

## Core Algorithm Reference

The estimator makes a single pass over the input, classifying characters inline and grouping runs of the same kind into segments. No regex, no allocations.

1. **Classify each character** as whitespace, punctuation, or word
2. **Group consecutive same-kind characters** into segments
3. **Score each segment** by type:

| Segment Type | Detection | Token Count |
|--------------|-----------|-------------|
| Whitespace | `char::is_whitespace()` | 0 |
| CJK characters | Unicode range checks | 1 per character |
| Digit sequences | `char::is_ascii_digit()` | 1 |
| Short words (≤3 bytes) | Length check | 1 |
| Punctuation | Match table | `ceil(len / 2)` |
| German/French diacritics | `is_german(c) \|\| is_french(c)` | `ceil(len / 3)` |
| Spanish diacritics | `is_spanish(c)` | `ceil(len / 3.5)` |
| Default alphanumeric | Fallback | `ceil(len / 6)` |
| Other (emojis, mixed) | Fallback | 1 per character |
