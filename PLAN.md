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

- [ ] Initialize Rust project structure
  - [ ] Run `cargo init --lib`
  - [ ] Configure `Cargo.toml` with metadata
  - [ ] Create directory structure (see below)
  - [ ] Add `.rustfmt.toml` for formatting preferences

- [ ] Set up GitHub Actions CI/CD
  - [ ] `.github/workflows/ci.yml` - Test, clippy, rustfmt on PRs
  - [ ] `.github/workflows/publish.yml` - Publish to crates.io on release tag

### Phase 2: Core Implementation

- [ ] Implement pattern matching (`src/patterns.rs`)
  - [ ] Whitespace detection regex
  - [ ] CJK character range regex (Chinese, Japanese, Korean)
  - [ ] Numeric pattern regex
  - [ ] Punctuation pattern regex
  - [ ] Alphanumeric pattern regex
  - [ ] Language-specific diacritics (German/French, Slavic)
  - [ ] Token split pattern (whitespace + punctuation)
  - [ ] Use `once_cell::sync::Lazy` for compiled patterns

- [ ] Implement configuration types (`src/config.rs`)
  - [ ] `LanguageConfig` struct (pattern, chars_per_token)
  - [ ] `EstimationOptions` struct (default_chars_per_token, language_configs)
  - [ ] `SplitOptions` struct (extends EstimationOptions with overlap)
  - [ ] Default language configurations (German, French, Slavic)
  - [ ] Builder pattern for options

- [ ] Implement core estimator (`src/estimator.rs`)
  - [ ] `estimate_token_count(text: &str) -> usize`
  - [ ] `estimate_token_count_with_options(text: &str, options: &EstimationOptions) -> usize`
  - [ ] `estimate_segment_tokens()` internal function
  - [ ] `get_language_chars_per_token()` internal function

- [ ] Implement utility functions (`src/utils.rs`)
  - [ ] `is_within_token_limit(text: &str, limit: usize) -> bool`
  - [ ] `slice_by_tokens(text: &str, start: usize, end: Option<usize>) -> String`
  - [ ] `split_by_tokens(text: &str, tokens_per_chunk: usize) -> Vec<String>`
  - [ ] Support negative indices in slice_by_tokens (like Python slicing)
  - [ ] Support overlap in split_by_tokens

- [ ] Create public API (`src/lib.rs`)
  - [ ] Re-export all public types and functions
  - [ ] Module documentation with examples
  - [ ] Feature flags (if any)

### Phase 3: Testing

- [ ] Unit tests (`src/*.rs` inline tests)
  - [ ] Empty string handling
  - [ ] Pure whitespace
  - [ ] Pure CJK text
  - [ ] Pure punctuation
  - [ ] Mixed content
  - [ ] Numeric strings
  - [ ] Short words (≤3 chars)
  - [ ] Language-specific text (German, French, Slavic)

- [ ] Integration tests (`tests/accuracy.rs`)
  - [ ] Port all tokenx benchmark cases
  - [ ] Add test fixtures for known texts
  - [ ] Compare against tiktoken-rs for ground truth validation
  - [ ] Document accuracy per test case

- [ ] Property-based tests (`tests/proptest.rs`)
  - [ ] Arbitrary string input doesn't panic
  - [ ] Result is always >= 0
  - [ ] Empty input returns 0
  - [ ] slice_by_tokens round-trip properties
  - [ ] split_by_tokens concatenation properties

- [ ] Benchmarks (`benches/estimation.rs`)
  - [ ] Short text (~20 tokens)
  - [ ] Medium text (~1000 tokens)
  - [ ] Long text (~30000 tokens)
  - [ ] CJK text
  - [ ] Code/TypeScript
  - [ ] Compare with tiktoken-rs performance

### Phase 4: Documentation

- [x] README.md
  - [x] Badges (crates.io version, docs.rs, license)
  - [x] One-line description
  - [x] Credit to original tokenx project
  - [x] Installation instructions
  - [x] Quick start example
  - [x] Accuracy benchmarks table
  - [x] License section

- [ ] Rustdoc documentation
  - [ ] Crate-level documentation with examples
  - [ ] All public functions documented
  - [ ] All public types documented
  - [ ] Examples for each major function
  - [ ] Links to related items

- [ ] CHANGELOG.md
  - [ ] Follow Keep a Changelog format
  - [ ] Document v0.1.0 initial release

- [x] LICENSE
  - [x] MIT license text

### Phase 5: Publishing

- [ ] Pre-publish validation
  - [ ] Run `cargo fmt --check`
  - [ ] Run `cargo clippy -- -D warnings`
  - [ ] Run `cargo test`
  - [ ] Run `cargo doc --no-deps`
  - [ ] Run `cargo publish --dry-run`
  - [ ] Verify all metadata in Cargo.toml

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
│   ├── estimator.rs               # Core estimation logic
│   ├── patterns.rs                # Compiled regex patterns
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

## Cargo.toml

```toml
[package]
name = "tokenx-rs"
version = "0.1.0"
edition = "2021"
rust-version = "1.70"
authors = ["Qbit AI"]
description = "Fast token count estimation for LLMs at 96% accuracy without a full tokenizer"
license = "MIT"
repository = "https://github.com/qbit-ai/tokenx-rs"
documentation = "https://docs.rs/tokenx-rs"
homepage = "https://github.com/qbit-ai/tokenx-rs"
readme = "README.md"
keywords = ["llm", "tokenizer", "tokens", "gpt", "claude"]
categories = ["text-processing", "algorithms"]
exclude = [".github/", "benches/", "tests/"]

[dependencies]
once_cell = "1.19"
regex = "1.10"

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
proptest = "1.4"
tiktoken-rs = "0.9"  # For accuracy validation only

[[bench]]
name = "estimation"
harness = false
```

---

## GitHub Actions: CI Workflow

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo test --all-features

  clippy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - uses: Swatinem/rust-cache@v2
      - run: cargo clippy --all-features -- -D warnings

  fmt:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - run: cargo fmt --check

  docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo doc --no-deps
        env:
          RUSTDOCFLAGS: -D warnings
```

---

## GitHub Actions: Publish Workflow

```yaml
# .github/workflows/publish.yml
name: Publish

on:
  release:
    types: [published]

env:
  CARGO_TERM_COLOR: always

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo publish
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

---

## Core Algorithm Reference

The algorithm works by:

1. **Split text** on whitespace and punctuation
2. **Classify each segment** and apply appropriate token estimation:

| Segment Type | Detection | Token Count |
|--------------|-----------|-------------|
| Whitespace | `^\s+$` | 0 |
| CJK characters | Unicode ranges | 1 per character |
| Numbers | `^\d+([.,]\d+)*$` | 1 |
| Short words (≤3 chars) | Length check | 1 |
| Punctuation | Character class | `ceil(len / 2)` |
| German/French diacritics | `[äöüßéèêë...]` | `ceil(len / 3)` |
| Slavic diacritics | `[ąćęłń...]` | `ceil(len / 3.5)` |
| Default alphanumeric | Fallback | `ceil(len / 6)` |
