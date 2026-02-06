//! Accuracy tests ported from the original tokenx project.

use tokenx_rs::estimate_token_count;

/// Helper: check that estimated tokens are within a given deviation percentage.
fn assert_within_deviation(label: &str, actual: usize, estimated: usize, max_deviation_pct: f64) {
    let deviation = if actual == 0 {
        if estimated == 0 {
            0.0
        } else {
            100.0
        }
    } else {
        ((estimated as f64 - actual as f64) / actual as f64).abs() * 100.0
    };
    assert!(
        deviation <= max_deviation_pct,
        "{label}: estimated {estimated} vs actual {actual}, deviation {deviation:.2}% exceeds {max_deviation_pct}%"
    );
}

#[test]
fn short_english_text() {
    // "The quick brown fox jumps over the lazy dog" — 9 words, all short
    // Our estimator counts each short word (≤3 chars) or word (≤6 chars) as 1 token
    let text = "The quick brown fox jumps over the lazy dog";
    let estimated = estimate_token_count(text);
    assert_eq!(estimated, 9);
}

#[test]
fn german_text_with_umlauts() {
    let text = "Der Äpfel fällt nicht weit vom Stamm. Übung macht den Meister. \
                Günter öffnete vorsichtig die Tür und blickte in den dunklen Raum.";
    let estimated = estimate_token_count(text);
    // Just check it's reasonable (non-zero, not wildly off)
    assert!(
        estimated > 10,
        "Expected >10 tokens for German text, got {estimated}"
    );
    assert!(
        estimated < 100,
        "Expected <100 tokens for German text, got {estimated}"
    );
}

#[test]
fn chinese_text() {
    // Each CJK character should be ~1 token
    let text = "道可道非常道名可名非常名";
    let estimated = estimate_token_count(text);
    let char_count = text.chars().count();
    assert_within_deviation("chinese", char_count, estimated, 15.0);
}

#[test]
fn japanese_text() {
    // Mix of kanji and katakana
    let text = "吾輩は猫である。名前はまだ無い。";
    let estimated = estimate_token_count(text);
    assert!(estimated > 0, "Expected >0 tokens for Japanese text");
}

#[test]
fn pure_numbers() {
    // Standalone integers = 1 token
    assert_eq!(estimate_token_count("42"), 1);
    // "3.14" splits on "." → "3" + "." + "14" = 3 tokens
    assert_eq!(estimate_token_count("3.14"), 3);
    // "1,000" splits on "," → "1" + "," + "000" = 3 tokens
    assert_eq!(estimate_token_count("1,000"), 3);
}

#[test]
fn code_snippet() {
    let code = r#"
fn main() {
    let x: i32 = 42;
    println!("Hello, world! The answer is {}", x);
}
"#;
    let estimated = estimate_token_count(code);
    assert!(
        estimated > 10,
        "Code should produce several tokens, got {estimated}"
    );
}

#[test]
fn mixed_language_content() {
    let text = "Hello 你好 Bonjour مرحبا";
    let estimated = estimate_token_count(text);
    assert!(
        estimated >= 4,
        "Mixed language should produce multiple tokens, got {estimated}"
    );
}

#[test]
fn empty_and_whitespace() {
    assert_eq!(estimate_token_count(""), 0);
    assert_eq!(estimate_token_count("   "), 0);
    assert_eq!(estimate_token_count("\n\n\n"), 0);
}

#[test]
fn single_character() {
    assert!(estimate_token_count("a") > 0);
    assert!(estimate_token_count("!") > 0);
}
