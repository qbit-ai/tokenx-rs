//! # tokenx-rs
//!
//! Fast token count estimation for LLMs at 96% accuracy without a full tokenizer.
//!
//! This is a Rust port of [tokenx](https://github.com/johannschopplich/tokenx) by
//! Johann Schopplich. It uses heuristic rules to estimate how many tokens a piece of
//! text will consume when sent to an LLM, without needing any vocabulary files.
//!
//! ## Quick start
//!
//! ```
//! use tokenx_rs::estimate_token_count;
//!
//! let tokens = estimate_token_count("Hello, world!");
//! assert!(tokens > 0);
//! ```
//!
//! ## When to use this
//!
//! - **Token budget estimation** before sending requests to an LLM API.
//! - **Streaming display** of approximate token counts in real time.
//! - **Pre-flight checks** to see if a prompt fits a model's context window.
//!
//! For exact counts, use a full BPE tokenizer like `tiktoken-rs`.

mod config;
mod estimator;
mod utils;

pub use config::{EstimationOptions, LanguageConfig, SplitOptions};
pub use estimator::{estimate_token_count, estimate_token_count_with_options};
pub use utils::{
    is_within_token_limit, is_within_token_limit_with_options, slice_by_tokens,
    slice_by_tokens_with_options, split_by_tokens, split_by_tokens_with_options,
};
