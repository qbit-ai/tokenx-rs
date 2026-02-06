//! Core token estimation logic.
//!
//! Single-pass scanner that classifies characters inline using simple
//! char-level checks. Zero allocations, zero regex, zero dependencies.

use crate::config::EstimationOptions;

// ── char classification helpers ─────────────────────────────────────────

#[inline(always)]
fn is_cjk(c: char) -> bool {
    matches!(c,
        '\u{4E00}'..='\u{9FFF}'   // CJK Unified Ideographs
        | '\u{3400}'..='\u{4DBF}' // CJK Unified Ideographs Extension A
        | '\u{3000}'..='\u{303F}' // CJK Symbols and Punctuation
        | '\u{FF00}'..='\u{FFEF}' // Halfwidth and Fullwidth Forms
        | '\u{30A0}'..='\u{30FF}' // Katakana
        | '\u{2E80}'..='\u{2EFF}' // CJK Radicals Supplement
        | '\u{31C0}'..='\u{31EF}' // CJK Strokes
        | '\u{3200}'..='\u{32FF}' // Enclosed CJK Letters and Months
        | '\u{3300}'..='\u{33FF}' // CJK Compatibility
        | '\u{AC00}'..='\u{D7AF}' // Hangul Syllables
        | '\u{1100}'..='\u{11FF}' // Hangul Jamo
        | '\u{3130}'..='\u{318F}' // Hangul Compatibility Jamo
        | '\u{A960}'..='\u{A97F}' // Hangul Jamo Extended-A
        | '\u{D7B0}'..='\u{D7FF}' // Hangul Jamo Extended-B
    )
}

#[inline(always)]
fn is_punctuation(c: char) -> bool {
    matches!(
        c,
        '.' | ','
            | '!'
            | '?'
            | ';'
            | '\''
            | '"'
            | '\u{201E}' // „
            | '\u{201C}' // "
            | '\u{201D}' // "
            | '\u{2018}' // '
            | '\u{2019}' // '
            | '-'
            | '('
            | ')'
            | '{'
            | '}'
            | '['
            | ']'
            | '<'
            | '>'
            | ':'
            | '/'
            | '\\'
            | '|'
            | '@'
            | '#'
            | '$'
            | '%'
            | '^'
            | '&'
            | '*'
            | '+'
            | '='
            | '`'
            | '~'
    )
}

#[inline(always)]
fn is_alphanumeric_latin(c: char) -> bool {
    c.is_ascii_alphanumeric()
        || matches!(c, '\u{00C0}'..='\u{00D6}' | '\u{00D8}'..='\u{00F6}' | '\u{00F8}'..='\u{00FF}')
}

// ── split boundary classification ───────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
enum SplitKind {
    Whitespace,
    Punctuation,
    Word,
}

#[inline(always)]
fn split_classify(c: char) -> SplitKind {
    if c.is_whitespace() {
        SplitKind::Whitespace
    } else if c.is_ascii() {
        if is_punctuation(c) {
            SplitKind::Punctuation
        } else {
            SplitKind::Word
        }
    } else if is_punctuation(c) {
        SplitKind::Punctuation
    } else {
        SplitKind::Word
    }
}

// ── segment scoring ─────────────────────────────────────────────────────

#[inline(always)]
fn score_word(
    byte_len: usize,
    char_count: usize,
    has_cjk: bool,
    all_alphanum: bool,
    all_digits: bool,
    lang_cpt: Option<f64>,
    default_cpt: f64,
) -> usize {
    if has_cjk {
        return char_count;
    }
    if all_digits {
        return 1;
    }
    if byte_len <= 3 {
        return 1;
    }
    if all_alphanum || lang_cpt.is_some() {
        let cpt = lang_cpt.unwrap_or(default_cpt);
        return (byte_len as f64 / cpt).ceil() as usize;
    }
    char_count
}

#[inline(always)]
fn score_punctuation(byte_len: usize) -> usize {
    if byte_len <= 3 {
        1
    } else {
        (byte_len + 1) / 2
    }
}

// ── public API ──────────────────────────────────────────────────────────

/// Estimates the number of tokens in `text` using default options.
///
/// # Examples
///
/// ```
/// use tokenx_rs::estimate_token_count;
///
/// assert_eq!(estimate_token_count(""), 0);
/// assert!(estimate_token_count("Hello, world!") > 0);
/// ```
pub fn estimate_token_count(text: &str) -> usize {
    estimate_token_count_with_options(text, &EstimationOptions::default())
}

/// Estimates the number of tokens in `text` using custom options.
///
/// # Examples
///
/// ```
/// use tokenx_rs::{estimate_token_count_with_options, EstimationOptions};
///
/// let opts = EstimationOptions::default();
/// let tokens = estimate_token_count_with_options("Hello, world!", &opts);
/// assert!(tokens > 0);
/// ```
pub fn estimate_token_count_with_options(text: &str, options: &EstimationOptions) -> usize {
    if text.is_empty() {
        return 0;
    }

    let mut total_tokens: usize = 0;

    let mut seg_split_kind = SplitKind::Word;
    let mut seg_byte_len: usize = 0;
    let mut seg_char_count: usize = 0;
    let mut seg_has_cjk = false;
    let mut seg_all_alphanum = true;
    let mut seg_all_digits = true;
    let mut seg_lang_cpt: Option<f64> = None;
    let mut in_segment = false;

    let default_cpt = options.default_chars_per_token;

    macro_rules! flush {
        () => {
            total_tokens += match seg_split_kind {
                SplitKind::Whitespace => 0,
                SplitKind::Punctuation => score_punctuation(seg_byte_len),
                SplitKind::Word => score_word(
                    seg_byte_len,
                    seg_char_count,
                    seg_has_cjk,
                    seg_all_alphanum,
                    seg_all_digits,
                    seg_lang_cpt,
                    default_cpt,
                ),
            };
        };
    }

    for c in text.chars() {
        let kind = split_classify(c);

        if in_segment && kind == seg_split_kind {
            seg_byte_len += c.len_utf8();
            seg_char_count += 1;
            if kind == SplitKind::Word {
                if is_cjk(c) {
                    seg_has_cjk = true;
                }
                if !is_alphanumeric_latin(c) {
                    seg_all_alphanum = false;
                    seg_all_digits = false;
                } else if !c.is_ascii_digit() {
                    seg_all_digits = false;
                }
                if seg_lang_cpt.is_none() {
                    seg_lang_cpt = detect_language_cpt(c, options);
                }
            }
        } else {
            if in_segment {
                flush!();
            }
            seg_split_kind = kind;
            seg_byte_len = c.len_utf8();
            seg_char_count = 1;
            seg_has_cjk = kind == SplitKind::Word && is_cjk(c);
            seg_all_alphanum = kind != SplitKind::Word || is_alphanumeric_latin(c);
            seg_all_digits = kind == SplitKind::Word && c.is_ascii_digit();
            seg_lang_cpt = if kind == SplitKind::Word {
                detect_language_cpt(c, options)
            } else {
                None
            };
            in_segment = true;
        }
    }

    if in_segment {
        flush!();
    }

    total_tokens
}

#[inline(always)]
fn detect_language_cpt(c: char, options: &EstimationOptions) -> Option<f64> {
    for lc in &options.language_configs {
        if (lc.matcher)(c) {
            return Some(lc.chars_per_token);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DEFAULT_CHARS_PER_TOKEN;

    #[test]
    fn empty_string() {
        assert_eq!(estimate_token_count(""), 0);
    }

    #[test]
    fn pure_whitespace() {
        assert_eq!(estimate_token_count("   "), 0);
        assert_eq!(estimate_token_count("\t\n"), 0);
    }

    #[test]
    fn pure_cjk() {
        assert_eq!(estimate_token_count("你好世界"), 4);
    }

    #[test]
    fn pure_punctuation() {
        assert_eq!(estimate_token_count("..."), 1);
        assert_eq!(estimate_token_count(","), 1);
    }

    #[test]
    fn numeric_string() {
        assert_eq!(estimate_token_count("12345"), 1);
        assert_eq!(estimate_token_count("3.14"), 3);
    }

    #[test]
    fn short_words() {
        assert_eq!(estimate_token_count("Hi Bob"), 2);
    }

    #[test]
    fn mixed_content() {
        let count = estimate_token_count("Hello, world!");
        assert!(count >= 2, "Expected at least 2 tokens, got {count}");
    }

    #[test]
    fn german_text() {
        let count = estimate_token_count("Ärgerlich");
        assert!(count > 0);
    }

    #[test]
    fn french_text() {
        let count = estimate_token_count("résumé");
        assert!(count > 0);
    }

    #[test]
    fn english_sentence() {
        let count = estimate_token_count("The quick brown fox jumps over the lazy dog");
        assert!(count >= 9, "Expected at least 9 tokens, got {count}");
    }

    #[test]
    fn default_chars_per_token_constant() {
        assert_eq!(DEFAULT_CHARS_PER_TOKEN, 6.0);
    }

    #[test]
    fn underscore_identifiers() {
        let count = estimate_token_count("process_items");
        assert_eq!(count, 13); // 1 per char (not alphanumeric due to _)
    }

    #[test]
    fn custom_options() {
        let opts = EstimationOptions {
            default_chars_per_token: 4.0,
            language_configs: vec![],
        };
        let count = estimate_token_count_with_options("abcdefgh", &opts);
        // ceil(8 / 4.0) = 2
        assert_eq!(count, 2);
    }
}
