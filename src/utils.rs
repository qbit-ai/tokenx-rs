//! Utility functions built on top of the token estimator.

use crate::config::{EstimationOptions, SplitOptions};
use crate::estimator::estimate_token_count_with_options;

/// Returns `true` if the estimated token count of `text` is at most `limit`.
///
/// # Examples
///
/// ```
/// use tokenx_rs::is_within_token_limit;
///
/// assert!(is_within_token_limit("Hello", 100));
/// assert!(!is_within_token_limit("Hello", 0));
/// ```
pub fn is_within_token_limit(text: &str, limit: usize) -> bool {
    is_within_token_limit_with_options(text, limit, &EstimationOptions::default())
}

/// Returns `true` if the estimated token count of `text` (with custom options) is at most `limit`.
pub fn is_within_token_limit_with_options(
    text: &str,
    limit: usize,
    options: &EstimationOptions,
) -> bool {
    estimate_token_count_with_options(text, options) <= limit
}

/// Extracts a substring from `text` by estimated token positions.
///
/// - `start` is the token index to begin at (0-based). Negative values count from the end.
/// - `end` is the exclusive upper token index. `None` means "to the end". Negative values
///   count from the end.
///
/// The function re-estimates token boundaries by walking the text word-by-word, so the
/// result is approximate.
///
/// # Examples
///
/// ```
/// use tokenx_rs::slice_by_tokens;
///
/// let text = "The quick brown fox jumps over the lazy dog";
/// let sliced = slice_by_tokens(text, 0, Some(3));
/// assert!(!sliced.is_empty());
/// ```
pub fn slice_by_tokens(text: &str, start: isize, end: Option<isize>) -> String {
    slice_by_tokens_with_options(text, start, end, &EstimationOptions::default())
}

/// Like [`slice_by_tokens`] but with custom estimation options.
pub fn slice_by_tokens_with_options(
    text: &str,
    start: isize,
    end: Option<isize>,
    options: &EstimationOptions,
) -> String {
    if text.is_empty() {
        return String::new();
    }

    let total = estimate_token_count_with_options(text, options);
    if total == 0 {
        return String::new();
    }

    let resolve = |idx: isize| -> usize {
        if idx < 0 {
            total.saturating_sub((-idx) as usize)
        } else {
            (idx as usize).min(total)
        }
    };

    let abs_start = resolve(start);
    let abs_end = match end {
        Some(e) => resolve(e),
        None => total,
    };

    if abs_start >= abs_end {
        return String::new();
    }

    // Walk through words, accumulating tokens, and collect those in [abs_start, abs_end).
    let mut result = String::new();
    let mut current_token = 0usize;

    for word in text.split_inclusive(char::is_whitespace) {
        let word_tokens = estimate_token_count_with_options(word, options);
        let word_end = current_token + word_tokens;

        if word_end > abs_start && current_token < abs_end {
            result.push_str(word);
        }

        current_token = word_end;
        if current_token >= abs_end {
            break;
        }
    }

    result
}

/// Splits `text` into chunks of approximately `tokens_per_chunk` tokens each.
///
/// # Examples
///
/// ```
/// use tokenx_rs::split_by_tokens;
///
/// let chunks = split_by_tokens("one two three four five six seven eight nine ten", 3);
/// assert!(chunks.len() >= 2);
/// ```
pub fn split_by_tokens(text: &str, tokens_per_chunk: usize) -> Vec<String> {
    split_by_tokens_with_options(text, tokens_per_chunk, &SplitOptions::default())
}

/// Like [`split_by_tokens`] but with custom split options (including overlap).
pub fn split_by_tokens_with_options(
    text: &str,
    tokens_per_chunk: usize,
    options: &SplitOptions,
) -> Vec<String> {
    if text.is_empty() || tokens_per_chunk == 0 {
        return vec![];
    }

    let words: Vec<&str> = text.split_inclusive(char::is_whitespace).collect();
    let mut chunks = Vec::new();
    let mut i = 0;

    while i < words.len() {
        let mut chunk = String::new();
        let mut chunk_tokens = 0usize;
        let start_i = i;

        while i < words.len() && chunk_tokens < tokens_per_chunk {
            let word_tokens = estimate_token_count_with_options(words[i], &options.estimation);
            chunk.push_str(words[i]);
            chunk_tokens += word_tokens;
            i += 1;
        }

        if !chunk.is_empty() {
            chunks.push(chunk);
        }

        // Handle overlap by rewinding.
        if options.overlap > 0 && i < words.len() {
            let mut overlap_tokens = 0usize;
            let mut rewind = 0;
            let mut j = i;
            while j > start_i && overlap_tokens < options.overlap {
                j -= 1;
                overlap_tokens += estimate_token_count_with_options(words[j], &options.estimation);
                rewind += 1;
            }
            i -= rewind;
        }
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn within_limit_basic() {
        assert!(is_within_token_limit("Hello", 100));
        assert!(!is_within_token_limit("Hello", 0));
    }

    #[test]
    fn slice_empty() {
        assert_eq!(slice_by_tokens("", 0, None), "");
    }

    #[test]
    fn slice_full_range() {
        let text = "Hello world";
        let sliced = slice_by_tokens(text, 0, None);
        assert_eq!(sliced, text);
    }

    #[test]
    fn slice_negative_start() {
        let text = "one two three four five";
        let sliced = slice_by_tokens(text, -2, None);
        assert!(!sliced.is_empty());
    }

    #[test]
    fn slice_inverted_range() {
        assert_eq!(slice_by_tokens("hello world", 5, Some(2)), "");
    }

    #[test]
    fn split_basic() {
        let chunks = split_by_tokens("one two three four five six", 2);
        assert!(
            chunks.len() >= 2,
            "Expected multiple chunks, got {}",
            chunks.len()
        );
    }

    #[test]
    fn split_empty() {
        assert!(split_by_tokens("", 5).is_empty());
    }

    #[test]
    fn split_zero_chunk_size() {
        assert!(split_by_tokens("hello", 0).is_empty());
    }

    #[test]
    fn split_with_overlap() {
        let opts = SplitOptions {
            overlap: 1,
            ..Default::default()
        };
        let chunks =
            split_by_tokens_with_options("one two three four five six seven eight", 3, &opts);
        assert!(chunks.len() >= 2);
        // With overlap, chunks should share some content.
    }
}
