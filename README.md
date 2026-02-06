# tokenx-rs

> Rust port of [tokenx](https://github.com/johannschopplich/tokenx) by [Johann Schopplich](https://github.com/johannschopplich)

Fast token count estimation for LLMs at 96% accuracy without a full tokenizer.

[![Crates.io](https://img.shields.io/crates/v/tokenx-rs.svg)](https://crates.io/crates/tokenx-rs)
[![Documentation](https://docs.rs/tokenx-rs/badge.svg)](https://docs.rs/tokenx-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Why?

- **No vocabulary files**: Full tokenizers like tiktoken require 2-4MB of BPE vocabulary data
- **Zero dependencies**: No regex, no allocations — just a single-pass character scanner
- **Fast**: ~8x faster than the original Node.js implementation on Latin text
- **Accurate enough**: 96% accuracy is sufficient for token budget estimation and streaming display
- **Universal**: Works across all LLM providers (OpenAI, Anthropic, Google, etc.)

## Installation

```toml
[dependencies]
tokenx-rs = "0.1"
```

## Usage

```rust
use tokenx_rs::estimate_token_count;

let tokens = estimate_token_count("Hello, world!");
println!("Estimated tokens: {}", tokens);
```

### Check token limits

```rust
use tokenx_rs::is_within_token_limit;

if is_within_token_limit(prompt, 4096) {
    // safe to send
}
```

### Split text into chunks

```rust
use tokenx_rs::split_by_tokens;

let chunks = split_by_tokens(long_text, 1000);
```

## How it works

The estimator makes a single pass over the input string, classifying each character and grouping runs of similar characters into segments. Each segment is scored by type:

| Segment Type | Rule |
|---|---|
| Whitespace | 0 tokens |
| CJK characters | 1 token per character |
| Digit sequences | 1 token |
| Short words (≤3 bytes) | 1 token |
| Punctuation runs | `ceil(len / 2)` |
| Alphanumeric words | `ceil(len / chars_per_token)` |
| Other (emojis, mixed) | 1 token per character |

Language-specific diacritics (German, French, Spanish) adjust the `chars_per_token` ratio for more accurate estimates on non-English text.

There is no regex, no string splitting allocation, and no runtime dependencies.

## Performance

Benchmarked against the original [tokenx](https://github.com/johannschopplich/tokenx) Node.js library on the same machine (Apple Silicon, Node v20, rustc 1.91):

| Input | tokenx (Node.js) | tokenx-rs (Rust) | Speedup |
|---|---:|---:|---:|
| Short text (~9 words) | 830 ns | 107 ns | **7.8x** |
| Medium text (~900 words) | 89.6 µs | 10.9 µs | **8.2x** |
| Long text (~27k words) | 2.91 ms | 332 µs | **8.8x** |
| CJK text (1200 chars) | 6.34 µs | 4.04 µs | **1.6x** |
| Code (~100 fn blocks) | 443 µs | 52.9 µs | **8.4x** |

Run benchmarks yourself with `cargo bench`.

## Accuracy

Accuracy benchmarks from the original tokenx project (compared against tiktoken cl100k_base):

| Content | Actual Tokens | Estimated | Deviation |
|---------|---------------|-----------|-----------|
| Short English text | 19 | 19 | 0.00% |
| German text with umlauts | 48 | 49 | 2.08% |
| Kafka - Metamorphosis (English) | 31,796 | 32,325 | 1.66% |
| Kafka - Die Verwandlung (German) | 35,309 | 33,970 | 3.79% |
| 道德經 - Laozi (Chinese) | 11,712 | 11,427 | 2.43% |
| 羅生門 - Akutagawa (Japanese) | 9,517 | 10,535 | 10.70% |
| TypeScript ES5 declarations | 49,293 | 51,599 | 4.68% |

## License

MIT
