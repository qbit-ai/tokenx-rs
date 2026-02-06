//! Property-based tests for tokenx-rs.

use proptest::prelude::*;
use tokenx_rs::{estimate_token_count, is_within_token_limit, slice_by_tokens, split_by_tokens};

proptest! {
    #[test]
    fn never_panics_on_arbitrary_input(s in ".*") {
        let _ = estimate_token_count(&s);
    }

    #[test]
    fn result_is_consistent(s in ".{0,500}") {
        let a = estimate_token_count(&s);
        let b = estimate_token_count(&s);
        prop_assert_eq!(a, b, "Same input should always produce same result");
    }

    #[test]
    fn empty_input_returns_zero(s in r"\s*") {
        let count = estimate_token_count(&s);
        // Pure whitespace should return 0
        prop_assert_eq!(count, 0, "Pure whitespace '{}' should return 0, got {}", s, count);
    }

    #[test]
    fn is_within_limit_consistent(s in ".{0,200}", limit in 0usize..1000) {
        let count = estimate_token_count(&s);
        let within = is_within_token_limit(&s, limit);
        prop_assert_eq!(within, count <= limit);
    }

    #[test]
    fn slice_by_tokens_subset(s in "[a-zA-Z0-9][a-zA-Z0-9 ]{0,200}[a-zA-Z0-9]") {
        // Only test on strings that produce >0 tokens (exclude pure whitespace)
        let full = slice_by_tokens(&s, 0, None);
        prop_assert_eq!(full, s);
    }

    #[test]
    fn split_by_tokens_covers_all_content(s in "[a-z ]{10,200}", chunk in 1usize..20) {
        let chunks = split_by_tokens(&s, chunk);
        if s.trim().is_empty() {
            return Ok(());
        }
        // Joining chunks should approximately reconstruct the input
        let joined: String = chunks.join("");
        // At minimum, the joined output should have similar length (overlap may add duplication)
        prop_assert!(joined.len() >= s.len() / 2, "Chunks seem to lose too much content");
    }

    #[test]
    fn split_produces_nonempty_chunks(s in "[a-z]{5,100}", chunk in 1usize..10) {
        let chunks = split_by_tokens(&s, chunk);
        for c in &chunks {
            prop_assert!(!c.is_empty(), "Chunks should never be empty");
        }
    }
}
